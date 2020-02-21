mod battery;

use crate::config::OutputConfig;
use crate::dbusdata::battery::*;
use crate::error::Error;
use crate::generated::dbus_properties::DBusPropertiesPropertiesChanged;
use crate::generated::upower::UPower;
use crate::output::{OutputUpdate, UpdateType};

use dbus::arg::RefArg;
use dbus::blocking::Connection;
use dbus::message::Message;
use log::{debug, error};
use std::sync::mpsc::Sender;
use std::time::Duration;

pub struct DBusdata {
    conn: Connection,
    batteries: Vec<BatterySource>,
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
                let bat = battery::BatterySource::new(id, conf, &self.conn)?;
                self.batteries.push(bat);
            }
            _ => {}
        };

        Ok(self)
    }

    pub fn start_listening(&mut self, tx: Sender<OutputUpdate>) -> Result<(), Error> {
        let proxy_timeout = Duration::from_secs(5);

        if !self.batteries.is_empty() {
            debug!("Setup charging state listener");

            let upower_proxy = self.conn.with_proxy(
                "org.freedesktop.UPower",
                "/org/freedesktop/UPower",
                proxy_timeout,
            );

            let bat_ids = self.batteries.iter().map(|b| b.id).collect();
            let handler = create_discharging_state_handler(tx.clone(), bat_ids);
            upower_proxy.match_signal(handler)?;

            let on_battery = upower_proxy.on_battery()?;
            for bat in self.batteries.iter() {
                bat.start_listening(tx.clone(), &self.conn)?;
                tx.send(OutputUpdate {
                    id: bat.id,
                    update: UpdateType::OnBattery(on_battery),
                })?;
            }
        };

        loop {
            self.conn.process(Duration::from_secs(1))?;
        }
    }
}

fn create_discharging_state_handler<'a>(
    tx: Sender<OutputUpdate>,
    ids: Vec<usize>,
) -> impl FnMut(DBusPropertiesPropertiesChanged, &Connection, &Message) -> bool + 'a {
    move |props: DBusPropertiesPropertiesChanged, _: &Connection, _: &Message| {
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
