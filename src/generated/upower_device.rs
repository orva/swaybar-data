// This code was autogenerated with `dbus-codegen-rust --system-bus --generic-variant --methodtype None --destination org.freedesktop.UPower --skipprefix org.freedesktop --interfaces org.freedesktop.UPower.Device --path /org/freedesktop/UPower/devices/DisplayDevice`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait UPowerDevice {
    fn refresh(&self) -> Result<(), dbus::Error>;
    fn get_history(&self, type_: &str, timespan: u32, resolution: u32) -> Result<Vec<(u32, f64, u32)>, dbus::Error>;
    fn get_statistics(&self, type_: &str) -> Result<Vec<(f64, f64)>, dbus::Error>;
    fn native_path(&self) -> Result<String, dbus::Error>;
    fn vendor(&self) -> Result<String, dbus::Error>;
    fn model(&self) -> Result<String, dbus::Error>;
    fn serial(&self) -> Result<String, dbus::Error>;
    fn update_time(&self) -> Result<u64, dbus::Error>;
    fn type_(&self) -> Result<u32, dbus::Error>;
    fn power_supply(&self) -> Result<bool, dbus::Error>;
    fn has_history(&self) -> Result<bool, dbus::Error>;
    fn has_statistics(&self) -> Result<bool, dbus::Error>;
    fn online(&self) -> Result<bool, dbus::Error>;
    fn energy(&self) -> Result<f64, dbus::Error>;
    fn energy_empty(&self) -> Result<f64, dbus::Error>;
    fn energy_full(&self) -> Result<f64, dbus::Error>;
    fn energy_full_design(&self) -> Result<f64, dbus::Error>;
    fn energy_rate(&self) -> Result<f64, dbus::Error>;
    fn voltage(&self) -> Result<f64, dbus::Error>;
    fn luminosity(&self) -> Result<f64, dbus::Error>;
    fn time_to_empty(&self) -> Result<i64, dbus::Error>;
    fn time_to_full(&self) -> Result<i64, dbus::Error>;
    fn percentage(&self) -> Result<f64, dbus::Error>;
    fn temperature(&self) -> Result<f64, dbus::Error>;
    fn is_present(&self) -> Result<bool, dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn is_rechargeable(&self) -> Result<bool, dbus::Error>;
    fn capacity(&self) -> Result<f64, dbus::Error>;
    fn technology(&self) -> Result<u32, dbus::Error>;
    fn warning_level(&self) -> Result<u32, dbus::Error>;
    fn battery_level(&self) -> Result<u32, dbus::Error>;
    fn icon_name(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> UPowerDevice for blocking::Proxy<'a, C> {

    fn refresh(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.UPower.Device", "Refresh", ())
    }

    fn get_history(&self, type_: &str, timespan: u32, resolution: u32) -> Result<Vec<(u32, f64, u32)>, dbus::Error> {
        self.method_call("org.freedesktop.UPower.Device", "GetHistory", (type_, timespan, resolution, ))
            .and_then(|r: (Vec<(u32, f64, u32)>, )| Ok(r.0, ))
    }

    fn get_statistics(&self, type_: &str) -> Result<Vec<(f64, f64)>, dbus::Error> {
        self.method_call("org.freedesktop.UPower.Device", "GetStatistics", (type_, ))
            .and_then(|r: (Vec<(f64, f64)>, )| Ok(r.0, ))
    }

    fn native_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "NativePath")
    }

    fn vendor(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Vendor")
    }

    fn model(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Model")
    }

    fn serial(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Serial")
    }

    fn update_time(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "UpdateTime")
    }

    fn type_(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Type")
    }

    fn power_supply(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "PowerSupply")
    }

    fn has_history(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "HasHistory")
    }

    fn has_statistics(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "HasStatistics")
    }

    fn online(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Online")
    }

    fn energy(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Energy")
    }

    fn energy_empty(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "EnergyEmpty")
    }

    fn energy_full(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "EnergyFull")
    }

    fn energy_full_design(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "EnergyFullDesign")
    }

    fn energy_rate(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "EnergyRate")
    }

    fn voltage(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Voltage")
    }

    fn luminosity(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Luminosity")
    }

    fn time_to_empty(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "TimeToEmpty")
    }

    fn time_to_full(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "TimeToFull")
    }

    fn percentage(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Percentage")
    }

    fn temperature(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Temperature")
    }

    fn is_present(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "IsPresent")
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "State")
    }

    fn is_rechargeable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "IsRechargeable")
    }

    fn capacity(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Capacity")
    }

    fn technology(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "Technology")
    }

    fn warning_level(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "WarningLevel")
    }

    fn battery_level(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "BatteryLevel")
    }

    fn icon_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.UPower.Device", "IconName")
    }
}
