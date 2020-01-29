use crate::config::OutputConfig;
use crate::error::Error;
use crate::generated::upower::OrgFreedesktopUPower;
use crate::generated::upower_device::OrgFreedesktopDBusPropertiesPropertiesChanged as UPowerDevPropsChanged;
use crate::generated::upower_device::OrgFreedesktopUPowerDevice;
use crate::{OutputUpdate, UpdateType};

use dbus::arg::RefArg;
use dbus::blocking::Connection;
use dbus::message::Message;
use log::{debug, error};
use std::sync::mpsc::Sender;
use std::time::Duration;

#[derive(PartialEq)]
pub enum DeviceType {
    Unknown,
    LinePower,
    Battery,
    Ups,
    Monitor,
    Mouse,
    Keyboard,
    Pda,
    Phone,
}

impl From<u32> for DeviceType {
    fn from(val: u32) -> Self {
        match val {
            1 => DeviceType::LinePower,
            2 => DeviceType::Battery,
            3 => DeviceType::Ups,
            4 => DeviceType::Monitor,
            5 => DeviceType::Mouse,
            6 => DeviceType::Keyboard,
            7 => DeviceType::Pda,
            8 => DeviceType::Phone,
            _ => DeviceType::Unknown,
        }
    }
}

pub struct DBusdata {
    conn: Connection,
    batteries: Vec<Battery>,
}

struct Battery {
    id: usize,
    path: dbus::Path<'static>,
}

impl DBusdata {
    pub fn new() -> Result<Self, Error> {
        let conn = Connection::new_system()?;
        Ok(DBusdata {
            conn,
            batteries: Vec::new(),
        })
    }

    pub fn with_config(&mut self, config: (usize, OutputConfig)) -> Result<&mut Self, Error> {
        let (id, conf) = config;
        match conf {
            OutputConfig::Battery => {
                let proxy_timeout = Duration::from_secs(5);
                let upower = self.conn.with_proxy(
                    "org.freedesktop.UPower",
                    "/org/freedesktop/UPower",
                    proxy_timeout,
                );
                let bat_proxy = upower
                    .enumerate_devices()?
                    .into_iter()
                    .map(|path| {
                        self.conn
                            .with_proxy("org.freedesktop.UPower", path, proxy_timeout)
                    })
                    .find(|proxy| match proxy.get_type() {
                        Err(_) => {
                            error!("Failed to get device type");
                            false
                        }
                        Ok(dt) => DeviceType::from(dt) == DeviceType::Battery,
                    })
                    .ok_or(Error::NoBatteryFound)?;

                let battery = Battery {
                    id,
                    path: bat_proxy.path,
                };

                self.batteries.push(battery);
            }
            _ => {}
        };

        Ok(self)
    }

    pub fn start_listening(&mut self, tx: Sender<OutputUpdate>) -> Result<(), Error> {
        for bat in self.batteries.iter() {
            debug!("Setup battery listener for {:?}", bat.path);
            let tx = tx.clone();
            let proxy_timeout = Duration::from_secs(5);
            let bat_proxy =
                self.conn
                    .with_proxy("org.freedesktop.UPower", &bat.path, proxy_timeout);
            let percentage = bat_proxy.get_percentage()?;
            tx.send(OutputUpdate {
                id: bat.id,
                update: UpdateType::Percentage(percentage),
            })?;

            let handler = create_battery_handler(tx, bat.id);
            bat_proxy.match_signal(handler)?;
        }

        loop {
            self.conn.process(Duration::from_secs(1))?;
        }
    }
}

fn create_battery_handler<'a>(
    tx: Sender<OutputUpdate>,
    id: usize,
) -> impl FnMut(UPowerDevPropsChanged, &Connection, &Message) -> bool + 'a {
    move |props: UPowerDevPropsChanged, _: &Connection, _: &Message| {
        if let Some(arg) = props.changed_properties.get("Percentage") {
            let percentage = match arg.as_f64() {
                Some(p) => p,
                None => {
                    error!("Percentage could not read as f64, terminating listener");
                    return false;
                }
            };

            debug!("Sending new battery percentage {}", percentage);

            let update = UpdateType::Percentage(percentage);
            if let Err(err) = tx.send(OutputUpdate { id, update }) {
                error!("Sending battery update failed with {}", err);
                return false;
            }
        }

        true
    }
}
