#!/usr/bin/env bash
#
# You need to provide IDs to required DBus objects, use busctl, dbus-send, or
# dbus client of your choice to find ones with required interface(s)

set -o errexit
set -o nounset
set -o pipefail

SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
pushd "$SCRIPTPATH" > /dev/null || exit
trap "popd > /dev/null" EXIT

UPOWER=${UPOWER:-0}
NETWORK_MANAGER=${NETWORK_MANAGER:-0}
ACTIVE_CONNECTION_ID=${ACTIVE_CONNECTION_ID:-0}
WIRELESS_DEVICE_ID=${WIRELESS_DEVICE_ID:-0}
WIRED_DEVICE_ID=${WIRED_DEVICE_ID:-0}
ACCESS_POINT_ID=${ACCESS_POINT_ID:-0}

if [[ "$UPOWER" != "0" ]]; then
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
fi

if [[ "$NETWORK_MANAGER" != "0" ]]; then
    echo "## Generating NetworkManager"
    dbus-codegen-rust --system-bus --generic-variant --methodtype None  \
        --destination org.freedesktop.NetworkManager                    \
        --skipprefix org.freedesktop                                    \
        --interfaces org.freedesktop.NetworkManager                     \
        --path /org/freedesktop/NetworkManager > ../src/generated/network_manager.rs
fi

if [[ "$ACTIVE_CONNECTION_ID" != "0" ]]; then
    echo "## Generating NetworkManager.Connection.Active"
    dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
        --destination org.freedesktop.NetworkManager                        \
        --skipprefix org.freedesktop                                        \
        --interfaces org.freedesktop.NetworkManager.Connection.Active       \
        --path "/org/freedesktop/NetworkManager/ActiveConnection/${ACTIVE_CONNECTION_ID}" > ../src/generated/nm_active_connection.rs
fi

if [[ "$WIRELESS_DEVICE_ID" != "0" ]]; then
    echo "## Generating NetworkManager.Device"
    dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
        --destination org.freedesktop.NetworkManager                        \
        --skipprefix org.freedesktop                                        \
        --interfaces org.freedesktop.NetworkManager.Device                  \
        --path "/org/freedesktop/NetworkManager/Devices/${WIRELESS_DEVICE_ID}" > ../src/generated/nm_device.rs

    echo "## Generating NetworkManager.Device.Wireless"
    dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
        --destination org.freedesktop.NetworkManager                        \
        --skipprefix org.freedesktop                                        \
        --interfaces org.freedesktop.NetworkManager.Device.Wireless         \
        --path "/org/freedesktop/NetworkManager/Devices/${WIRELESS_DEVICE_ID}" > ../src/generated/nm_device_wireless.rs
fi

if [[ "$WIRED_DEVICE_ID" != "0" ]]; then
    echo "## Generating NetworkManager.Device.Wired"
    dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
        --destination org.freedesktop.NetworkManager                        \
        --skipprefix org.freedesktop                                        \
        --interfaces org.freedesktop.NetworkManager.Device.Wired            \
        --path "/org/freedesktop/NetworkManager/Devices/${WIRED_DEVICE_ID}" > ../src/generated/nm_device_wired.rs
fi

if [[ "$ACCESS_POINT_ID" != "0" ]]; then
    echo "## Generating NetworkManager.AccessPoint"
    dbus-codegen-rust --system-bus --generic-variant --methodtype None      \
        --destination org.freedesktop.NetworkManager                        \
        --skipprefix org.freedesktop                                        \
        --interfaces org.freedesktop.NetworkManager.AccessPoint             \
        --path "/org/freedesktop/NetworkManager/AccessPoint/${ACCESS_POINT_ID}" > ../src/generated/nm_access_point.rs
fi

