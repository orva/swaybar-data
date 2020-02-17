use crate::config::OutputConfig;
use crate::error::Error;
use crate::generated::upower::DBusPropertiesPropertiesChanged as UPowerPropsChanged;
use crate::generated::upower::UPower;
use crate::generated::upower_device::DBusPropertiesPropertiesChanged as UPowerDevPropsChanged;
use crate::generated::upower_device::UPowerDevice;
use crate::output::{OutputUpdate, UpdateType};

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
        let proxy_timeout = Duration::from_secs(5);

        let on_battery = if self.batteries.is_empty() {
            false
        } else {
            debug!("Setup charging state listener");

            let upower_proxy = self.conn.with_proxy(
                "org.freedesktop.UPower",
                "/org/freedesktop/UPower",
                proxy_timeout,
            );
            let bat_ids = self.batteries.iter().map(|b| b.id).collect();
            let handler = create_discharging_state_handler(tx.clone(), bat_ids);
            upower_proxy.match_signal(handler)?;
            upower_proxy.get_on_battery()?
        };

        for bat in self.batteries.iter() {
            debug!("Setup battery listener for {:?}", bat.path);

            let tx = tx.clone();
            let bat_proxy =
                self.conn
                    .with_proxy("org.freedesktop.UPower", &bat.path, proxy_timeout);

            let percentage = bat_proxy.get_percentage()?;
            tx.send(OutputUpdate {
                id: bat.id,
                update: UpdateType::Percentage(percentage),
            })?;

            let time_to_full = bat_proxy.get_time_to_full()?;
            tx.send(OutputUpdate {
                id: bat.id,
                update: UpdateType::TimeToFull(time_to_full),
            })?;

            let time_to_empty = bat_proxy.get_time_to_empty()?;
            tx.send(OutputUpdate {
                id: bat.id,
                update: UpdateType::TimeToEmpty(time_to_empty),
            })?;

            tx.send(OutputUpdate {
                id: bat.id,
                update: UpdateType::OnBattery(on_battery),
            })?;

            let handler = create_battery_change_handler(tx, bat.id);
            bat_proxy.match_signal(handler)?;
        }

        loop {
            self.conn.process(Duration::from_secs(1))?;
        }
    }
}

fn create_battery_change_handler<'a>(
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

        if let Some(arg) = props.changed_properties.get("TimeToFull") {
            let time_to_full = match arg.as_i64() {
                Some(t) => t,
                None => {
                    error!("TimeToFull could not be read as i64, terminating listener");
                    return false;
                }
            };

            debug!("Sending new battery time_to_full {}", time_to_full);

            let update = UpdateType::TimeToFull(time_to_full);
            if let Err(err) = tx.send(OutputUpdate { id, update }) {
                error!("Sending battery update failed with {}", err);
                return false;
            }
        }

        if let Some(arg) = props.changed_properties.get("TimeToEmpty") {
            let time_to_empty = match arg.as_i64() {
                Some(t) => t,
                None => {
                    error!("TimeToEmpty could not be read as i64, terminating listener");
                    return false;
                }
            };

            debug!("Sending new battery time_to_empty {}", time_to_empty);

            let update = UpdateType::TimeToEmpty(time_to_empty);
            if let Err(err) = tx.send(OutputUpdate { id, update }) {
                error!("Sending battery update failed with {}", err);
                return false;
            }
        }

        true
    }
}

fn create_discharging_state_handler<'a>(
    tx: Sender<OutputUpdate>,
    ids: Vec<usize>,
) -> impl FnMut(UPowerPropsChanged, &Connection, &Message) -> bool + 'a {
    move |props: UPowerPropsChanged, _: &Connection, _: &Message| {
        if let Some(arg) = props.changed_properties.get("OnBattery") {
            let on_battery = match arg.as_u64() {
                Some(p) => p != 0,
                None => {
                    error!("OnBattery could not be read as u64, terminating listener");
                    return false;
                }
            };

            debug!("Sending new on_battery state {}", on_battery);

            for id in ids.clone() {
                let update = UpdateType::OnBattery(on_battery);
                if let Err(err) = tx.send(OutputUpdate { id, update }) {
                    error!("Sending battery update failed with {}", err);
                    return false;
                }
            }
        }

        true
    }
}
