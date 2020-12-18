use crate::utils::device::{Device, DeviceBuilder};
use crate::utils::conn_util::{with_conn, ConnVariant};
use crate::mconnect_dbus::OrgMconnectDeviceManager;
use std::path::PathBuf;

pub struct DeviceManager;

impl OrgMconnectDeviceManager for DeviceManager {
    fn allow_device(&self, device:&str) -> Result<(), dbus::Error> { 
        with_conn(ConnVariant::DeviceManager, |p| p.allow_device(device)).and_then(|e|e)
    }
    fn disallow_device(&self, device:&str) -> Result<(), dbus::Error> {
        with_conn(ConnVariant::DeviceManager, |p| p.disallow_device(device)).and_then(|e|e)
    }
    fn list_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        with_conn(ConnVariant::DeviceManager, |p| p.list_devices()).and_then(|e|e)
    }
    fn certificate(&self) -> Result<std::string::String, dbus::Error> {
        with_conn(ConnVariant::DeviceManager, |p| p.certificate()).and_then(|e|e)
    }
    fn set_certificate(&self, certificate:String) -> Result<(), dbus::Error> {
        with_conn(ConnVariant::DeviceManager, |p| p.set_certificate(certificate.clone())).and_then(|e|e)
    }
}

impl DeviceManager {
    pub fn list_device_structs(&self) -> Result<Vec<Device>, dbus::Error> {
        Ok(self.list_devices()?
            .iter()
            .map(|path| 
                    with_conn(
                        ConnVariant::Device(path), 
                        |p| DeviceBuilder::default().from_proxy(PathBuf::from(path.to_string()), p).build().unwrap()).unwrap())
            .collect())
    }
}