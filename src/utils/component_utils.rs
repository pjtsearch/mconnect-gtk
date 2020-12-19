use vgtk::lib::gtk::*;

pub fn class<M, C>(widget: &C, class:&str, message:M) -> M where C : WidgetExt {
    widget.get_style_context().add_class(class);
    message
}