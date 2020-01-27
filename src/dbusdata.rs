use crate::generated::upower::OrgFreedesktopUPower;
use crate::generated::upower_device::OrgFreedesktopUPowerDevice;
use crate::OutputUpdate;

use dbus::blocking::Connection;
use log::info;
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

pub struct DBusData {
    tx: Sender<OutputUpdate>,
    conn: Connection,
}

impl DBusData {
    pub fn new(tx: Sender<OutputUpdate>, id: usize) -> Self {
        let conn = Connection::new_system().unwrap();
        let p = conn.with_proxy(
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower",
            Duration::from_secs(5),
        );
        let dev_paths = p.enumerate_devices().unwrap();
        let battery = dev_paths
            .iter()
            .map(|p| conn.with_proxy("org.freedesktop.UPower", p, Duration::from_secs(5)))
            .find(|dp| {
                let dev_type = DeviceType::from(dp.get_type().unwrap());
                dev_type == DeviceType::Battery
            })
            .unwrap();

        let percentage = battery.get_percentage().unwrap().floor() as i64;
        info!("battery capacity {:?}", percentage);

        tx.send(OutputUpdate(percentage.to_string(), id)).unwrap();

        DBusData { tx, conn }
    }
}
