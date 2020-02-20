#!/usr/bin/env bash
#
# NOTE: to run this script, it needs some editing!
#
# You need to provide IDs to required DBus objects, use busctl, dbus-send, or
# dbus client of your choice to find ones with required interface

set -o errexit
set -o nounset
set -o pipefail

SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
pushd "$SCRIPTPATH" > /dev/null || exit
trap "popd > /dev/null" EXIT

WIRELESS_DEVICE_ID=2
WIRED_DEVICE_ID=3
ACCESS_POINT_ID=302

echo "## Generating DBus.Properties (from UPower)"
dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
    --destination org.freedesktop.UPower                            \
    --skipprefix org.freedesktop                                    \
    --interfaces org.freedesktop.DBus.Properties                    \
    --path /org/freedesktop/UPower > ../src/generated/dbus_properties.rs

echo "## Generating UPower"
dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
    --destination org.freedesktop.UPower                            \
    --skipprefix org.freedesktop                                    \
    --interfaces org.freedesktop.UPower                             \
    --path /org/freedesktop/UPower > ../src/generated/upower.rs

echo "## Generating UPower.Device"
dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
    --destination org.freedesktop.UPower                            \
    --skipprefix org.freedesktop                                    \
    --interfaces org.freedesktop.UPower.Device                      \
    --path /org/freedesktop/UPower/devices/DisplayDevice > ../src/generated/upower_device.rs

echo "## Generating NetworkManager"
dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
    --destination org.freedesktop.NetworkManager                    \
    --skipprefix org.freedesktop                                    \
    --interfaces org.freedesktop.NetworkManager                     \
    --path /org/freedesktop/NetworkManager > ../src/generated/network_manager.rs

echo "## Generating NetworkManager.Device"
dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
    --destination org.freedesktop.NetworkManager                        \
    --skipprefix org.freedesktop                                        \
    --interfaces org.freedesktop.NetworkManager.Device                  \
    --path /org/freedesktop/NetworkManager/Devices/${WIRELESS_DEVICE_ID} > ../src/generated/nm_device.rs

echo "## Generating NetworkManager.Device.Wireless"
dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
    --destination org.freedesktop.NetworkManager                        \
    --skipprefix org.freedesktop                                        \
    --interfaces org.freedesktop.NetworkManager.Device.Wireless         \
    --path /org/freedesktop/NetworkManager/Devices/${WIRELESS_DEVICE_ID} > ../src/generated/nm_device_wireless.rs

echo "## Generating NetworkManager.Device.Wired"
dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
    --destination org.freedesktop.NetworkManager                        \
    --skipprefix org.freedesktop                                        \
    --interfaces org.freedesktop.NetworkManager.Device.Wired            \
    --path /org/freedesktop/NetworkManager/Devices/${WIRED_DEVICE_ID} > ../src/generated/nm_device_wired.rs

echo "## Generating NetworkManager.AccessPoint"
dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
    --destination org.freedesktop.NetworkManager                        \
    --skipprefix org.freedesktop                                        \
    --interfaces org.freedesktop.NetworkManager.AccessPoint             \
    --path /org/freedesktop/NetworkManager/AccessPoint/${ACCESS_POINT_ID} > ../src/generated/nm_access_point.rs

