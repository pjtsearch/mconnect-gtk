use vgtk::lib::gtk::*;
use vgtk::lib::gdk::*;

pub fn class<M, C>(widget: &C, class:&str, message:M) -> M where C : WidgetExt {
    widget.get_style_context().add_class(class);
    message
}

pub fn css<M>(style: &str, message:M) -> M {
    let provider = CssProvider::new();
    provider
        .load_from_data(style.as_bytes())
        .expect("Failed to load CSS");
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    StyleContext::add_provider_for_screen(
        &Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    message
}