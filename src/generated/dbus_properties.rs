// This code was autogenerated with `dbus-codegen-rust --system-bus --generic-variant --methodtype None --destination org.freedesktop.UPower --skipprefix org.freedesktop --interfaces org.freedesktop.DBus.Properties --path /org/freedesktop/UPower`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait DBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> DBusProperties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, arg::Variant(value), ))
    }
}

#[derive(Debug)]
pub struct DBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for DBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for DBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}
