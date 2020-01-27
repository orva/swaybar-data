use crate::config::Output;
use crate::error::Error;
use crate::generated::upower::OrgFreedesktopUPower;
use crate::generated::upower_device::OrgFreedesktopDBusPropertiesPropertiesChanged as UPowerDevPropsChanged;
use crate::generated::upower_device::OrgFreedesktopUPowerDevice;
use crate::OutputUpdate;

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
    battery_paths: Vec<(usize, dbus::Path<'static>)>,
}

impl DBusdata {
    pub fn new() -> Result<Self, Error> {
        let conn = Connection::new_system()?;
        Ok(DBusdata {
            conn,
            battery_paths: Vec::new(),
        })
    }

    pub fn with_config(&mut self, config: (usize, Output)) -> Result<&mut Self, Error> {
        let (id, conf) = config;
        match conf {
            Output::Battery => {
                let upower = self.conn.with_proxy(
                    "org.freedesktop.UPower",
                    "/org/freedesktop/UPower",
                    Duration::from_secs(5),
                );
                let proxy_timeout = Duration::from_secs(5);
                let battery_path = upower
                    .enumerate_devices()?
                    .into_iter()
                    .map(|path| {
                        self.conn
                            .with_proxy("org.freedesktop.UPower", path, proxy_timeout)
                    })
                    .find(|dev_proxy| {
                        let dev_type = DeviceType::from(dev_proxy.get_type().unwrap());
                        dev_type == DeviceType::Battery
                    })
                    .map(|dev_proxy| dev_proxy.path)
                    .ok_or(Error::NoBatteryFound)?;

                self.battery_paths.push((id, battery_path));
            }
            _ => {}
        };

        Ok(self)
    }

    pub fn start_listening(self, tx: Sender<OutputUpdate>) -> Result<(), Error> {
        for (id, path) in self.battery_paths.into_iter() {
            debug!("Setuping battery listener for {:?}", path);
            let tx = tx.clone();
            let dev_timeout = Duration::from_secs(5);
            let battery = self
                .conn
                .with_proxy("org.freedesktop.UPower", path, dev_timeout);

            let start_percentage = battery.get_percentage().unwrap().floor() as i64;
            let update = start_percentage.to_string();
            tx.send(OutputUpdate { id, update })?;

            let handler = create_battery_handler(tx, id);
            battery.match_signal(handler)?;
        }

        let mut conn = self.conn;
        loop {
            conn.process(Duration::from_secs(1))?;
        }
    }
}

fn create_battery_handler(
    tx: Sender<OutputUpdate>,
    id: usize,
) -> impl FnMut(UPowerDevPropsChanged, &Connection, &Message) -> bool {
    move |props: UPowerDevPropsChanged, _: &Connection, _: &Message| {
        if let Some(arg) = props.changed_properties.get("Percentage") {
            let percentage = arg.as_f64().unwrap().floor() as i64;
            debug!("Sending new battery percentage {}", percentage);

            let update = percentage.to_string();
            return match tx.send(OutputUpdate { id, update }) {
                Ok(_) => true,
                Err(err) => {
                    error!("Sending battery update failed with {}", err);
                    false
                }
            };
        };
        true
    }
}
