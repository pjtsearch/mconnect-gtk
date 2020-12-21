use vgtk::lib::gtk::*;
use vgtk::lib::{gdk::Screen, glib::IsA, glib::GString};

pub fn class<M, C>(widget: &C, class:&str, message:M) -> M where C : WidgetExt {
    widget.get_style_context().add_class(class);
    message
}

pub trait GetSetCSS {
    fn set_css(&self, style:GString);
    fn get_css(&self) -> GString;
}

impl<O: IsA<Widget>> GetSetCSS for O {
    fn set_css(&self, style: GString) {
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
    }
    fn get_css(&self) -> GString {
        GString::from("")
    }
}