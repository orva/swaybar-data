//! Module for the code generated with `dbus-codegen-rust`
//!
//! Files were generated with following commands:
//!
//! ```bash
//! cargo install dbus-codegen
//! dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
//!     --destination org.freedesktop.UPower                            \
//!     --path /org/freedesktop/UPower/devices/DisplayDevice > src/generated/upower_device.rs
//! dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
//!     --destination org.freedesktop.UPower                            \
//!     --path /org/freedesktop/UPower > src/generated/upower.rs
//! ```
//!
//! Note that even though we are introspecting `DisplayDevice` it uses same
//! `org.freedesktop.UPower.Device` interface as battery/powersupply devices. Benefit of this is
//! that bindings can be generated on device which doesn't have batteries included.

#[rustfmt::skip]
pub mod upower;
#[rustfmt::skip]
pub mod upower_device;
