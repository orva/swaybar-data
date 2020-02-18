use crate::config::OutputConfig;
use crate::error::Error;
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
enum DeviceType {
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

pub struct Battery {
    pub id: usize,
    pub path: dbus::Path<'static>,
}

impl Battery {
    pub fn new(id: usize, _conf: OutputConfig, conn: &Connection) -> Result<Self, Error> {
        let proxy_timeout = Duration::from_secs(5);
        let upower = conn.with_proxy(
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower",
            proxy_timeout,
        );

        let bat_proxy = upower
            .enumerate_devices()?
            .into_iter()
            .map(|path| conn.with_proxy("org.freedesktop.UPower", path, proxy_timeout))
            .find(|proxy| match proxy.type_() {
                Err(_) => {
                    error!("Failed to get device type");
                    false
                }
                Ok(dt) => DeviceType::from(dt) == DeviceType::Battery,
            })
            .ok_or(Error::NoBatteryFound)?;

        Ok(Battery {
            id,
            path: bat_proxy.path,
        })
    }

    pub fn start_listening(
        &self,
        tx: Sender<OutputUpdate>,
        conn: &Connection,
    ) -> Result<(), Error> {
        debug!("Setup battery listener for {:?}", self.path);
        let proxy_timeout = Duration::from_secs(5);

        let bat_proxy = conn.with_proxy("org.freedesktop.UPower", &self.path, proxy_timeout);

        let percentage = bat_proxy.percentage()?;
        tx.send(OutputUpdate {
            id: self.id,
            update: UpdateType::Percentage(percentage),
        })?;

        let time_to_full = bat_proxy.time_to_full()?;
        tx.send(OutputUpdate {
            id: self.id,
            update: UpdateType::TimeToFull(time_to_full),
        })?;

        let time_to_empty = bat_proxy.time_to_empty()?;
        tx.send(OutputUpdate {
            id: self.id,
            update: UpdateType::TimeToEmpty(time_to_empty),
        })?;

        let handler = create_battery_change_handler(tx, self.id);
        bat_proxy.match_signal(handler)?;
        Ok(())
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
