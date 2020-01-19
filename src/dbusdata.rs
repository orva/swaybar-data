use crate::generated::*;

// use dbus::blocking::Connection;
// use dbus::Message;
// use std::error::Error;
// use std::time::Duration;


pub enum DeviceType {
    Unknown = 0,
    LinePower = 1,
    Battery = 2,
    Ups = 3,
    Monitor = 4,
    Mouse = 5,
    Keyboard = 6,
    Pda = 7,
    Phone = 8,
}
