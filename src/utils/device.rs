use std::path::PathBuf;
use dbus::arg::Variant;
use std::collections::HashMap;
use dbus::arg;
use dbus::arg::RefArg;

#[derive(Default, Builder, Debug)]
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
    pub certificate_fingerprint: String
}

impl DeviceBuilder {
    pub fn from_map(&mut self, path: PathBuf, map:HashMap<String, Variant<Box<dyn arg::RefArg>>>) -> &mut DeviceBuilder {
        self.id(map.get("Id").unwrap().as_str().unwrap())
            .path(path)
            .name(map.get("Name").unwrap().as_str().unwrap())
            .device_type(map.get("DeviceType").unwrap().as_str().unwrap())
            .protocol_version(map.get("ProtocolVersion").unwrap().as_i64().unwrap().to_owned())
            .address(map.get("Address").unwrap().as_str().unwrap())
            .is_paired(map.get("IsPaired").unwrap().0.as_any().downcast_ref::<bool>().unwrap().to_owned())
            .allowed(map.get("Allowed").unwrap().0.as_any().downcast_ref::<bool>().unwrap().to_owned())
            .is_active(map.get("IsActive").unwrap().0.as_any().downcast_ref::<bool>().unwrap().to_owned())
            .is_connected(map.get("IsConnected").unwrap().0.as_any().downcast_ref::<bool>().unwrap().to_owned())
            // TODO: add IncomingCapabilities and OutgoingCapabilities
            .incoming_capabilities(vec![])
            .outgoing_capabilities(vec![])
            .certificate(map.get("Certificate").unwrap().as_str().unwrap())
            .certificate_fingerprint(map.get("CertificateFingerprint").unwrap().as_str().unwrap())
    }
}