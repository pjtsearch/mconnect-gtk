#![allow(clippy::all)]
// This code was autogenerated with `dbus-codegen-rust -g -m None -d org.mconnect -p /org/mconnect/manager`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<R0, dbus::Error>;
    fn get_all(
        &self,
        interface_name: &str,
    ) -> Result<
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        dbus::Error,
    >;
    fn set<I2: arg::Arg + arg::Append>(
        &self,
        interface_name: &str,
        property_name: &str,
        value: I2,
    ) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusProperties
    for blocking::Proxy<'a, C>
{
    fn get<R0: for<'b> arg::Get<'b> + 'static>(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<R0, dbus::Error> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Get",
            (interface_name, property_name),
        )
        .and_then(|r: (arg::Variant<R0>,)| Ok((r.0).0))
    }

    fn get_all(
        &self,
        interface_name: &str,
    ) -> Result<
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
        dbus::Error,
    > {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "GetAll",
            (interface_name,),
        )
        .and_then(
            |r: (
                ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
            )| Ok(r.0),
        )
    }

    fn set<I2: arg::Arg + arg::Append>(
        &self,
        interface_name: &str,
        property_name: &str,
        value: I2,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Set",
            (interface_name, property_name, arg::Variant(value)),
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
{
    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusPeer
    for blocking::Proxy<'a, C>
{
    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgMconnectDeviceManager {
    fn allow_device(&self, path: &str) -> Result<(), dbus::Error>;
    fn disallow_device(&self, path: &str) -> Result<(), dbus::Error>;
    fn list_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn certificate(&self) -> Result<String, dbus::Error>;
    fn set_certificate(&self, value: String) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgMconnectDeviceManager
    for blocking::Proxy<'a, C>
{
    fn allow_device(&self, path: &str) -> Result<(), dbus::Error> {
        self.method_call("org.mconnect.DeviceManager", "AllowDevice", (path,))
    }

    fn disallow_device(&self, path: &str) -> Result<(), dbus::Error> {
        self.method_call("org.mconnect.DeviceManager", "DisallowDevice", (path,))
    }

    fn list_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.mconnect.DeviceManager", "ListDevices", ())
            .and_then(|r: (Vec<dbus::Path<'static>>,)| Ok(r.0))
    }

    fn certificate(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.DeviceManager",
            "Certificate",
        )
    }

    fn set_certificate(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.DeviceManager",
            "Certificate",
            value,
        )
    }
}

#[derive(Debug)]
pub struct OrgMconnectDeviceManagerDeviceAdded {
    pub path: String,
}

impl arg::AppendAll for OrgMconnectDeviceManagerDeviceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.path, i);
    }
}

impl arg::ReadAll for OrgMconnectDeviceManagerDeviceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgMconnectDeviceManagerDeviceAdded { path: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgMconnectDeviceManagerDeviceAdded {
    const NAME: &'static str = "DeviceAdded";
    const INTERFACE: &'static str = "org.mconnect.DeviceManager";
}

#[derive(Debug)]
pub struct OrgMconnectDeviceManagerDeviceRemoved {
    pub path: String,
}

impl arg::AppendAll for OrgMconnectDeviceManagerDeviceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.path, i);
    }
}

impl arg::ReadAll for OrgMconnectDeviceManagerDeviceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgMconnectDeviceManagerDeviceRemoved { path: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgMconnectDeviceManagerDeviceRemoved {
    const NAME: &'static str = "DeviceRemoved";
    const INTERFACE: &'static str = "org.mconnect.DeviceManager";
}

// This code was autogenerated with `dbus-codegen-rust -g -m None -d org.mconnect -p /org/mconnect/device/0`, see https://github.com/diwic/dbus-rs

pub trait OrgMconnectDeviceBattery {
    fn level(&self) -> Result<u32, dbus::Error>;
    fn set_level(&self, value: u32) -> Result<(), dbus::Error>;
    fn charging(&self) -> Result<bool, dbus::Error>;
    fn set_charging(&self, value: bool) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgMconnectDeviceBattery
    for blocking::Proxy<'a, C>
{
    fn level(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device.Battery",
            "Level",
        )
    }

    fn charging(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device.Battery",
            "Charging",
        )
    }

    fn set_level(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device.Battery",
            "Level",
            value,
        )
    }

    fn set_charging(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device.Battery",
            "Charging",
            value,
        )
    }
}

pub trait OrgMconnectDevicePing {}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgMconnectDevicePing
    for blocking::Proxy<'a, C>
{
}

#[derive(Debug)]
pub struct OrgMconnectDevicePingPing {}

impl arg::AppendAll for OrgMconnectDevicePingPing {
    fn append(&self, _: &mut arg::IterAppend) {}
}

impl arg::ReadAll for OrgMconnectDevicePingPing {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgMconnectDevicePingPing {})
    }
}

impl dbus::message::SignalArgs for OrgMconnectDevicePingPing {
    const NAME: &'static str = "Ping";
    const INTERFACE: &'static str = "org.mconnect.Device.Ping";
}

pub trait OrgMconnectDeviceShare {
    fn share_file(&self, path: &str) -> Result<(), dbus::Error>;
    fn share_url(&self, url: &str) -> Result<(), dbus::Error>;
    fn share_text(&self, text: &str) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgMconnectDeviceShare
    for blocking::Proxy<'a, C>
{
    fn share_file(&self, path: &str) -> Result<(), dbus::Error> {
        self.method_call("org.mconnect.Device.Share", "ShareFile", (path,))
    }

    fn share_url(&self, url: &str) -> Result<(), dbus::Error> {
        self.method_call("org.mconnect.Device.Share", "ShareUrl", (url,))
    }

    fn share_text(&self, text: &str) -> Result<(), dbus::Error> {
        self.method_call("org.mconnect.Device.Share", "ShareText", (text,))
    }
}

pub trait OrgMconnectDeviceTelephony {
    fn send_sms(&self, number: &str, message: &str) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgMconnectDeviceTelephony
    for blocking::Proxy<'a, C>
{
    fn send_sms(&self, number: &str, message: &str) -> Result<(), dbus::Error> {
        self.method_call(
            "org.mconnect.Device.Telephony",
            "SendSms",
            (number, message),
        )
    }
}

pub trait OrgMconnectDevice {
    fn id(&self) -> Result<String, dbus::Error>;
    fn set_id(&self, value: String) -> Result<(), dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn set_name(&self, value: String) -> Result<(), dbus::Error>;
    fn device_type(&self) -> Result<String, dbus::Error>;
    fn set_device_type(&self, value: String) -> Result<(), dbus::Error>;
    fn protocol_version(&self) -> Result<u32, dbus::Error>;
    fn set_protocol_version(&self, value: u32) -> Result<(), dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn set_address(&self, value: String) -> Result<(), dbus::Error>;
    fn is_paired(&self) -> Result<bool, dbus::Error>;
    fn set_is_paired(&self, value: bool) -> Result<(), dbus::Error>;
    fn allowed(&self) -> Result<bool, dbus::Error>;
    fn set_allowed(&self, value: bool) -> Result<(), dbus::Error>;
    fn is_active(&self) -> Result<bool, dbus::Error>;
    fn set_is_active(&self, value: bool) -> Result<(), dbus::Error>;
    fn is_connected(&self) -> Result<bool, dbus::Error>;
    fn set_is_connected(&self, value: bool) -> Result<(), dbus::Error>;
    fn incoming_capabilities(&self) -> Result<Vec<String>, dbus::Error>;
    fn set_incoming_capabilities(&self, value: Vec<String>) -> Result<(), dbus::Error>;
    fn outgoing_capabilities(&self) -> Result<Vec<String>, dbus::Error>;
    fn set_outgoing_capabilities(&self, value: Vec<String>) -> Result<(), dbus::Error>;
    fn certificate(&self) -> Result<String, dbus::Error>;
    fn set_certificate(&self, value: String) -> Result<(), dbus::Error>;
    fn certificate_fingerprint(&self) -> Result<String, dbus::Error>;
    fn set_certificate_fingerprint(&self, value: String) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgMconnectDevice
    for blocking::Proxy<'a, C>
{
    fn id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "Id",
        )
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "Name",
        )
    }

    fn device_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "DeviceType",
        )
    }

    fn protocol_version(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "ProtocolVersion",
        )
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "Address",
        )
    }

    fn is_paired(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "IsPaired",
        )
    }

    fn allowed(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "Allowed",
        )
    }

    fn is_active(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "IsActive",
        )
    }

    fn is_connected(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "IsConnected",
        )
    }

    fn incoming_capabilities(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "IncomingCapabilities",
        )
    }

    fn outgoing_capabilities(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "OutgoingCapabilities",
        )
    }

    fn certificate(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "Certificate",
        )
    }

    fn certificate_fingerprint(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.mconnect.Device",
            "CertificateFingerprint",
        )
    }

    fn set_id(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "Id",
            value,
        )
    }

    fn set_name(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "Name",
            value,
        )
    }

    fn set_device_type(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "DeviceType",
            value,
        )
    }

    fn set_protocol_version(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "ProtocolVersion",
            value,
        )
    }

    fn set_address(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "Address",
            value,
        )
    }

    fn set_is_paired(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "IsPaired",
            value,
        )
    }

    fn set_allowed(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "Allowed",
            value,
        )
    }

    fn set_is_active(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "IsActive",
            value,
        )
    }

    fn set_is_connected(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "IsConnected",
            value,
        )
    }

    fn set_incoming_capabilities(&self, value: Vec<String>) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "IncomingCapabilities",
            value,
        )
    }

    fn set_outgoing_capabilities(&self, value: Vec<String>) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "OutgoingCapabilities",
            value,
        )
    }

    fn set_certificate(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "Certificate",
            value,
        )
    }

    fn set_certificate_fingerprint(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.mconnect.Device",
            "CertificateFingerprint",
            value,
        )
    }
}
