use vgtk::lib::gtk::*;
use vgtk::lib::{gdk::Screen, glib::GString, glib::IsA};

/// A trait for widgets that have CSS classes
#[allow(clippy::ptr_arg)]
pub trait CSSWidget {
    /// Sets the CSS classes
    fn set_class(&self, classes: &Vec<String>);
    /// Gets the CSS classes
    fn get_class(&self) -> Vec<String>;
}

impl<O: IsA<Widget>> CSSWidget for O {
    fn set_class(&self, classes: &Vec<String>) {
        classes.iter().for_each(|class| {
            if !self.get_style_context().has_class(&class) {
                self.get_style_context().add_class(&class);
            }
        });
    }
    fn get_class(&self) -> Vec<String> {
        self.get_style_context()
            .list_classes()
            .iter()
            .map(|e| e.to_string())
            .collect()
    }
}

/// Trait for an Application that can have CSS
pub trait CSSApplication {
    /// Sets the CSS
    fn set_css(&self, style: GString);
    /// Gets the CSS
    fn get_css(&self) -> GString;
}

impl CSSApplication for Application {
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
