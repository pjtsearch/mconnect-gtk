use crate::utils::conn_util::{ConnVariant, with_conn};
use dbus::blocking::{BlockingSender, Proxy};
use crate::mconnect_dbus::{OrgMconnectDevice, OrgMconnectDeviceBattery, OrgMconnectDeviceShare, OrgMconnectDeviceTelephony, OrgMconnectDeviceManager};
use std::path::PathBuf;

#[derive(Default, Builder, Debug, Clone)]
#[builder(setter(into))]
pub struct Device {
    pub path: PathBuf,
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub protocol_version: i64,
    pub address: String,
    pub is_paired: bool,
    pub allowed: bool,
    pub is_active: bool,
    pub is_connected: bool,
    pub incoming_capabilities: Vec<String>,
    pub outgoing_capabilities: Vec<String>,
    pub certificate: String,
    pub certificate_fingerprint: String,
    pub battery_level: i64,
    pub battery_charging: bool
}

impl DeviceBuilder {
    pub fn from_proxy<'a, T: BlockingSender, C: ::std::ops::Deref<Target=T>>(&mut self, path: PathBuf, device: Proxy<C>) -> &mut DeviceBuilder {
        self.id = device.id().ok();
        self.path = Some(path);
        self.name = device.name().ok();
        self.device_type = device.device_type().ok();
        self.protocol_version = device.protocol_version().ok().map(|p|i64::from(p));
        self.address = device.address().ok();
        self.is_paired = device.is_paired().ok();
        self.allowed = device.allowed().ok();
        self.is_active = device.is_active().ok();
        self.is_connected = device.is_connected().ok();
        self.incoming_capabilities = device.incoming_capabilities().ok();
        self.outgoing_capabilities = device.outgoing_capabilities().ok();
        self.certificate = OrgMconnectDevice::certificate(&device).ok();
        self.certificate_fingerprint = device.certificate_fingerprint().ok();
        self.battery_level = device.level().ok().map(|p|i64::from(p));
        self.battery_charging = device.charging().ok();
        return self;
    }
}

impl Device {
    pub fn refreshed(&self) -> Device {
        with_conn(
            ConnVariant::Device(&self.path.to_string_lossy()), 
            |p| DeviceBuilder::default().from_proxy(self.path.clone(), p).build().unwrap())
    }

    pub fn allow(&self) -> std::result::Result<(), dbus::Error> {
        with_conn(
            ConnVariant::DeviceManager, 
            |p| p.allow_device(&self.path.to_string_lossy()))
    }

    pub fn disallow(&self) -> std::result::Result<(), dbus::Error> {
        with_conn(
            ConnVariant::DeviceManager, 
            |p| p.disallow_device(&self.path.to_string_lossy()))
    }
}

impl OrgMconnectDeviceShare for Device {
    fn share_file(&self, path: &str) -> std::result::Result<(), dbus::Error> {
        with_conn(
            ConnVariant::Device(&self.path.to_string_lossy()), 
            |p| p.share_file(path))
    }
    fn share_url(&self, url: &str) -> std::result::Result<(), dbus::Error> {
        with_conn(
            ConnVariant::Device(&self.path.to_string_lossy()), 
            |p| p.share_url(url))
    }
    fn share_text(&self, text: &str) -> std::result::Result<(), dbus::Error> {
        with_conn(
            ConnVariant::Device(&self.path.to_string_lossy()), 
            |p| p.share_text(text))
    }
}

impl OrgMconnectDeviceTelephony for Device {
    fn send_sms(&self, number: &str, message: &str) -> std::result::Result<(), dbus::Error> {
        with_conn(
            ConnVariant::Device(&self.path.to_string_lossy()), 
            |p| p.send_sms(number, message))
    }
}