use crate::utils::component_utils::*;
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct Notification {
    revealed: bool,
    pub text: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Close,
}

impl Component for Notification {
    type Message = Message;
    type Properties = Self;

    fn create(mut props: Self) -> Self {
        if props.text.is_some() {
            props.revealed = true;
        }
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        if self.text.is_some() {
            self.revealed = true;
        }
        UpdateAction::Render
    }

    fn update(&mut self, event: Message) -> UpdateAction<Self> {
        match event {
            Message::Close => {
                self.revealed = false;
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Revealer valign=Align::Start halign=Align::Center reveal_child=self.revealed>
                <Box
                    orientation=Orientation::Horizontal
                    spacing=18
                    class=vec!["app-notification".to_owned()]>
                        <Label
                            Box::pack_type=PackType::Start
                            Box::expand=false
                            Box::fill=true
                            Box::padding=18
                            text=self.text.clone().unwrap_or_default()/>
                        <Button
                            Box::pack_type=PackType::Start
                            Box::expand=false
                            Box::fill=true
                            Box::padding=18
                            image="window-close-symbolic"
                            relief=ReliefStyle::None
                            receives_default=true
                            on clicked=|_| Message::Close/>
                </Box>
            </Revealer>
        }
    }
}
