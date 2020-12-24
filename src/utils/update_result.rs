use vgtk::{Component, UpdateAction};

/// A trait for [Components](Component) that update returning a [Result] to be handled
pub trait UpdateResult<M> {
    fn update_result(&mut self, message: M) -> Result<UpdateAction<Self>, dbus::Error>
    where
        Self: Component;
}
