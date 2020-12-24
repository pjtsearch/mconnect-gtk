use dbus::Error;
use std::time::Duration;

/// n enum representing endpoints for a dbus connection
pub enum ConnVariant<'a> {
    DeviceManager,
    Device(&'a str),
}

/// Provides a [dbus::blocking::Connection] to a callback
pub fn with_conn<F, R>(variant: ConnVariant, action: F) -> Result<R, Error>
where
    F: Fn(dbus::blocking::Proxy<'_, &dbus::blocking::Connection>) -> R,
{
    let c = dbus::blocking::Connection::new_session()?;
    match variant {
        ConnVariant::DeviceManager => Ok(action(c.with_proxy(
            "org.mconnect",
            "/org/mconnect/manager",
            Duration::from_millis(10),
        ))),
        ConnVariant::Device(path) => Ok(action(c.with_proxy(
            "org.mconnect",
            path,
            Duration::from_millis(10),
        ))),
    }
}
