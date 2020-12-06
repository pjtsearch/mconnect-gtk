use std::time::Duration;

pub enum ConnVariant {
    DeviceManager
}

pub struct ConnUtil;

impl ConnUtil {
    pub fn create_conn<F>(variant:ConnVariant, mut action: F) 
        where F: FnMut(dbus::blocking::Proxy<'_, &dbus::blocking::Connection>) -> () {
        let c = dbus::blocking::Connection::new_session().unwrap();
        match variant {
            ConnVariant::DeviceManager => 
                action(c.with_proxy("org.mconnect", "/org/mconnect/manager", Duration::from_millis(10)))
        }
    }
}