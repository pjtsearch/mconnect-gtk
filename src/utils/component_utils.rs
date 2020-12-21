use vgtk::lib::gtk::*;
use vgtk::lib::{gdk::Screen, glib::IsA, glib::GString};

pub trait CSSWidget {
    fn set_css(&self, style:GString);
    fn get_css(&self) -> GString;
    fn set_class(&self, classes:&Vec<String>);
    fn get_class(&self) -> Vec<String>;
}

impl<O: IsA<Widget>> CSSWidget for O {
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
    fn set_class(&self, classes: &Vec<String>) {
        classes.iter().for_each(|class| {
            if !self.get_style_context().has_class(&class){
                self.get_style_context().add_class(&class);
            }
        });
    }
    fn get_class(&self) -> Vec<String> {
        self.get_style_context().list_classes().iter().map(|e|e.to_string()).collect()
    }
}