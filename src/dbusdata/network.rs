use crate::config::OutputConfig;
use crate::dbusdata::DBusSource;
use crate::error::Error;
use crate::output::{OutputUpdate, UpdateType};

use dbus::blocking::Connection;
use log::debug;
use std::sync::mpsc::Sender;
use std::time::Duration;
use uuid::Uuid;

pub struct ActiveConnectionsSource {
    pub id: usize,
    connections: Vec<ConnectionInfo>,
}

impl DBusSource for ActiveConnectionsSource {
    fn new(id: usize, _conf: OutputConfig, _conn: &Connection) -> Result<Self, Error> {
        Ok(ActiveConnectionsSource {
            id,
            connections: Vec::new(),
        })
    }

    fn start_listening(
        &mut self,
        tx: Sender<OutputUpdate>,
        conn: &Connection,
    ) -> Result<(), Error> {
        use crate::generated::network_manager::NetworkManager;

        let proxy_timeout = Duration::from_secs(5);
        let nm = conn.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            proxy_timeout,
        );

        let acs = nm.active_connections()?;
        debug!("ActiveConnections {:?}", acs);

        for ac_path in acs.iter() {
            let info = get_device_info(ac_path, conn)?;
            debug!("Device info {:?}", info);
            match info.dev_type {
                DeviceType::Wifi => {
                    let wifi_info = get_wifi_info(&info, &conn)?;
                    let connected = (info.uuid, wifi_info.0, wifi_info.1);
                    self.connections.push(info);
                    tx.send(OutputUpdate {
                        id: self.id,
                        update: UpdateType::WifiConnected(connected),
                    })?;
                }
                DeviceType::Unknown => {}
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ConnectionInfo {
    dev_type: DeviceType,
    dev_path: dbus::Path<'static>,
    uuid: Uuid,
}

/// Device types which we are interested about
/// from https://developer.gnome.org/NetworkManager/stable/nm-dbus-types.html#NMDeviceType
#[derive(Debug, PartialEq)]
enum DeviceType {
    Wifi,
    Unknown,
}

impl From<u32> for DeviceType {
    fn from(from: u32) -> Self {
        match from {
            2 => DeviceType::Wifi,
            _ => DeviceType::Unknown,
        }
    }
}

fn get_device_info(
    ac_path: &dbus::Path<'static>,
    conn: &Connection,
) -> Result<ConnectionInfo, Error> {
    use crate::generated::nm_active_connection::NetworkManagerConnectionActive;
    use crate::generated::nm_device::NetworkManagerDevice;

    let active_conn = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        ac_path,
        Duration::from_secs(5),
    );

    // TODO: what are these multiple devices? VPN? Handle them!
    let dev_path = active_conn.devices()?[0].clone();
    let dev = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        &dev_path,
        Duration::from_secs(5),
    );

    let dt = dev.device_type()?;
    let dev_type = DeviceType::from(dt);
    let info = ConnectionInfo {
        dev_type,
        dev_path,
        uuid: Uuid::new_v4(),
    };

    Ok(info)
}

fn get_wifi_info(
    conn_info: &ConnectionInfo,
    conn: &Connection,
) -> Result<(Option<String>, u8), Error> {
    use crate::generated::nm_access_point::NetworkManagerAccessPoint;
    use crate::generated::nm_device_wireless::NetworkManagerDeviceWireless;

    let wifi = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        &conn_info.dev_path,
        Duration::from_secs(5),
    );

    let ap_path = wifi.active_access_point()?;
    let ap = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        ap_path,
        Duration::from_secs(5),
    );

    let ssid_raw = ap.ssid()?;
    let ssid = String::from_utf8(ssid_raw).ok();
    let strength = ap.strength()?;
    Ok((ssid, strength))
}
