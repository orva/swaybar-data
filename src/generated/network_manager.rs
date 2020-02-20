// This code was autogenerated with `dbus-codegen-rust --system-bus --generic-variant --methodtype None --destination org.freedesktop.NetworkManager --skipprefix org.freedesktop --interfaces org.freedesktop.NetworkManager --path /org/freedesktop/NetworkManager`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait NetworkManager {
    fn reload(&self, flags: u32) -> Result<(), dbus::Error>;
    fn get_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_all_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_device_by_ip_iface(&self, iface: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn activate_connection(&self, connection: dbus::Path, device: dbus::Path, specific_object: dbus::Path) -> Result<dbus::Path<'static>, dbus::Error>;
    fn add_and_activate_connection(&self, connection: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>, device: dbus::Path, specific_object: dbus::Path) -> Result<(dbus::Path<'static>, dbus::Path<'static>), dbus::Error>;
    fn add_and_activate_connection2(&self, connection: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>, device: dbus::Path, specific_object: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(dbus::Path<'static>, dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>), dbus::Error>;
    fn deactivate_connection(&self, active_connection: dbus::Path) -> Result<(), dbus::Error>;
    fn sleep(&self, sleep: bool) -> Result<(), dbus::Error>;
    fn enable(&self, enable: bool) -> Result<(), dbus::Error>;
    fn get_permissions(&self) -> Result<::std::collections::HashMap<String, String>, dbus::Error>;
    fn set_logging(&self, level: &str, domains: &str) -> Result<(), dbus::Error>;
    fn get_logging(&self) -> Result<(String, String), dbus::Error>;
    fn check_connectivity(&self) -> Result<u32, dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn checkpoint_create(&self, devices: Vec<dbus::Path>, rollback_timeout: u32, flags: u32) -> Result<dbus::Path<'static>, dbus::Error>;
    fn checkpoint_destroy(&self, checkpoint: dbus::Path) -> Result<(), dbus::Error>;
    fn checkpoint_rollback(&self, checkpoint: dbus::Path) -> Result<::std::collections::HashMap<String, u32>, dbus::Error>;
    fn checkpoint_adjust_rollback_timeout(&self, checkpoint: dbus::Path, add_timeout: u32) -> Result<(), dbus::Error>;
    fn devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn all_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn checkpoints(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn networking_enabled(&self) -> Result<bool, dbus::Error>;
    fn wireless_enabled(&self) -> Result<bool, dbus::Error>;
    fn set_wireless_enabled(&self, value: bool) -> Result<(), dbus::Error>;
    fn wireless_hardware_enabled(&self) -> Result<bool, dbus::Error>;
    fn wwan_enabled(&self) -> Result<bool, dbus::Error>;
    fn set_wwan_enabled(&self, value: bool) -> Result<(), dbus::Error>;
    fn wwan_hardware_enabled(&self) -> Result<bool, dbus::Error>;
    fn wimax_enabled(&self) -> Result<bool, dbus::Error>;
    fn set_wimax_enabled(&self, value: bool) -> Result<(), dbus::Error>;
    fn wimax_hardware_enabled(&self) -> Result<bool, dbus::Error>;
    fn active_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn primary_connection(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn primary_connection_type(&self) -> Result<String, dbus::Error>;
    fn metered(&self) -> Result<u32, dbus::Error>;
    fn activating_connection(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn startup(&self) -> Result<bool, dbus::Error>;
    fn version(&self) -> Result<String, dbus::Error>;
    fn capabilities(&self) -> Result<u32, dbus::Error>;
    fn state_(&self) -> Result<u32, dbus::Error>;
    fn connectivity(&self) -> Result<u32, dbus::Error>;
    fn connectivity_check_available(&self) -> Result<bool, dbus::Error>;
    fn connectivity_check_enabled(&self) -> Result<bool, dbus::Error>;
    fn set_connectivity_check_enabled(&self, value: bool) -> Result<(), dbus::Error>;
    fn connectivity_check_uri(&self) -> Result<String, dbus::Error>;
    fn global_dns_configuration(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set_global_dns_configuration(&self, value: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> NetworkManager for blocking::Proxy<'a, C> {

    fn reload(&self, flags: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "Reload", (flags, ))
    }

    fn get_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "GetDevices", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn get_all_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "GetAllDevices", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn get_device_by_ip_iface(&self, iface: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "GetDeviceByIpIface", (iface, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn activate_connection(&self, connection: dbus::Path, device: dbus::Path, specific_object: dbus::Path) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "ActivateConnection", (connection, device, specific_object, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn add_and_activate_connection(&self, connection: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>, device: dbus::Path, specific_object: dbus::Path) -> Result<(dbus::Path<'static>, dbus::Path<'static>), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "AddAndActivateConnection", (connection, device, specific_object, ))
    }

    fn add_and_activate_connection2(&self, connection: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>, device: dbus::Path, specific_object: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(dbus::Path<'static>, dbus::Path<'static>, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "AddAndActivateConnection2", (connection, device, specific_object, options, ))
    }

    fn deactivate_connection(&self, active_connection: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "DeactivateConnection", (active_connection, ))
    }

    fn sleep(&self, sleep: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "Sleep", (sleep, ))
    }

    fn enable(&self, enable: bool) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "Enable", (enable, ))
    }

    fn get_permissions(&self) -> Result<::std::collections::HashMap<String, String>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "GetPermissions", ())
            .and_then(|r: (::std::collections::HashMap<String, String>, )| Ok(r.0, ))
    }

    fn set_logging(&self, level: &str, domains: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "SetLogging", (level, domains, ))
    }

    fn get_logging(&self) -> Result<(String, String), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "GetLogging", ())
    }

    fn check_connectivity(&self) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "CheckConnectivity", ())
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "state", ())
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn checkpoint_create(&self, devices: Vec<dbus::Path>, rollback_timeout: u32, flags: u32) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "CheckpointCreate", (devices, rollback_timeout, flags, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn checkpoint_destroy(&self, checkpoint: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "CheckpointDestroy", (checkpoint, ))
    }

    fn checkpoint_rollback(&self, checkpoint: dbus::Path) -> Result<::std::collections::HashMap<String, u32>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "CheckpointRollback", (checkpoint, ))
            .and_then(|r: (::std::collections::HashMap<String, u32>, )| Ok(r.0, ))
    }

    fn checkpoint_adjust_rollback_timeout(&self, checkpoint: dbus::Path, add_timeout: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager", "CheckpointAdjustRollbackTimeout", (checkpoint, add_timeout, ))
    }

    fn devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Devices")
    }

    fn all_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "AllDevices")
    }

    fn checkpoints(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Checkpoints")
    }

    fn networking_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "NetworkingEnabled")
    }

    fn wireless_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "WirelessEnabled")
    }

    fn wireless_hardware_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "WirelessHardwareEnabled")
    }

    fn wwan_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "WwanEnabled")
    }

    fn wwan_hardware_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "WwanHardwareEnabled")
    }

    fn wimax_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "WimaxEnabled")
    }

    fn wimax_hardware_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "WimaxHardwareEnabled")
    }

    fn active_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "ActiveConnections")
    }

    fn primary_connection(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "PrimaryConnection")
    }

    fn primary_connection_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "PrimaryConnectionType")
    }

    fn metered(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Metered")
    }

    fn activating_connection(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "ActivatingConnection")
    }

    fn startup(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Startup")
    }

    fn version(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Version")
    }

    fn capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Capabilities")
    }

    fn state_(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "State")
    }

    fn connectivity(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "Connectivity")
    }

    fn connectivity_check_available(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "ConnectivityCheckAvailable")
    }

    fn connectivity_check_enabled(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "ConnectivityCheckEnabled")
    }

    fn connectivity_check_uri(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "ConnectivityCheckUri")
    }

    fn global_dns_configuration(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager", "GlobalDnsConfiguration")
    }

    fn set_wireless_enabled(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager", "WirelessEnabled", value)
    }

    fn set_wwan_enabled(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager", "WwanEnabled", value)
    }

    fn set_wimax_enabled(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager", "WimaxEnabled", value)
    }

    fn set_connectivity_check_enabled(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager", "ConnectivityCheckEnabled", value)
    }

    fn set_global_dns_configuration(&self, value: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager", "GlobalDnsConfiguration", value)
    }
}

#[derive(Debug)]
pub struct NetworkManagerPropertiesChanged {
    pub properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for NetworkManagerPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for NetworkManagerPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(NetworkManagerPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for NetworkManagerPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
}

#[derive(Debug)]
pub struct NetworkManagerCheckPermissions {
}

impl arg::AppendAll for NetworkManagerCheckPermissions {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for NetworkManagerCheckPermissions {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(NetworkManagerCheckPermissions {
        })
    }
}

impl dbus::message::SignalArgs for NetworkManagerCheckPermissions {
    const NAME: &'static str = "CheckPermissions";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
}

#[derive(Debug)]
pub struct NetworkManagerStateChanged {
    pub state: u32,
}

impl arg::AppendAll for NetworkManagerStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.state, i);
    }
}

impl arg::ReadAll for NetworkManagerStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(NetworkManagerStateChanged {
            state: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for NetworkManagerStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
}

#[derive(Debug)]
pub struct NetworkManagerDeviceAdded {
    pub device_path: dbus::Path<'static>,
}

impl arg::AppendAll for NetworkManagerDeviceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.device_path, i);
    }
}

impl arg::ReadAll for NetworkManagerDeviceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(NetworkManagerDeviceAdded {
            device_path: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for NetworkManagerDeviceAdded {
    const NAME: &'static str = "DeviceAdded";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
}

#[derive(Debug)]
pub struct NetworkManagerDeviceRemoved {
    pub device_path: dbus::Path<'static>,
}

impl arg::AppendAll for NetworkManagerDeviceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.device_path, i);
    }
}

impl arg::ReadAll for NetworkManagerDeviceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(NetworkManagerDeviceRemoved {
            device_path: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for NetworkManagerDeviceRemoved {
    const NAME: &'static str = "DeviceRemoved";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
}
