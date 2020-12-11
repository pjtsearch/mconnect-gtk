use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum HeaderMsg {
}

pub struct Model {
    title: String
}

#[widget]
impl Widget for Header {
    fn model(title:String) -> Model {
        Model {title}
    }

    fn update(&mut self, _event: HeaderMsg) {
        
    }

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            title: Some(&self.model.title),
            show_close_button: true,
        }
    }
}


#[cfg(test)]
mod tests {
    use gtk::HeaderBarExt;
    use gtk_test::{assert_title};

    use super::Header;

    #[test]
    fn main_header() {
        let (_component, widgets) = relm::init_test::<Header>("Test".to_string()).expect("init_test failed");
        let label = widgets.titlebar;

        assert_title!(label, "Test");
    }
}