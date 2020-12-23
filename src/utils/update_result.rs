use vgtk::{Component, UpdateAction};

pub trait UpdateResult<M> {
    fn update_result(&mut self, message: M) -> Result<UpdateAction<Self>, dbus::Error>
    where
        Self: Component;
}
