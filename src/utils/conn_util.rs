use std::time::Duration;

pub enum ConnVariant<'a> {
    DeviceManager,
    Device(&'a str)
}

pub struct ConnUtil;

impl ConnUtil {
    pub fn with_conn<F, R>(variant:ConnVariant, action: F) -> R
        where F: Fn(dbus::blocking::Proxy<'_, &dbus::blocking::Connection>) -> R {
        let c = dbus::blocking::Connection::new_session().unwrap();
        match variant {
            ConnVariant::DeviceManager => 
                action(c.with_proxy("org.mconnect", "/org/mconnect/manager", Duration::from_millis(10))),
            ConnVariant::Device(path) => action(c.with_proxy("org.mconnect", path, Duration::from_millis(10)))
        }
    }
}