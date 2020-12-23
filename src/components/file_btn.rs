use std::path::PathBuf;
use vgtk::lib::gtk::*;
use vgtk::Callback;
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct FileBtn {
    pub on_selected: Callback<PathBuf>,
    pub label: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenDialog,
}

impl Component for FileBtn {
    type Message = Message;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, event: Message) -> UpdateAction<Self> {
        match event {
            Message::OpenDialog => {
                let dialog = FileChooserDialog::with_buttons::<Window>(
                    Some("Open File"),
                    None,
                    FileChooserAction::Open,
                    &[
                        ("_Cancel", ResponseType::Cancel),
                        ("_Open", ResponseType::Accept),
                    ],
                );
                let result = dialog.run();
                if result == ResponseType::Accept {
                    if let Some(uri) = dialog.get_filename() {
                        self.on_selected.send(uri);
                    }
                }
                dialog.close();
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Button label=self.label.clone() on clicked=|_| Message::OpenDialog />
        }
    }
}
