use vgtk::lib::gtk::*;
use vgtk::Callback;
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct InputBtn {
    pub on_input: Callback<String>,
    pub label: String,
    input: Option<String>,
    input_open: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    ToggleInput,
    Enter(String),
}

impl Component for InputBtn {
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
            Message::ToggleInput => {
                self.input_open = !self.input_open;
                UpdateAction::Render
            }
            Message::Enter(input) => {
                self.on_input.send(input);
                self.input_open = false;
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box>
                <Button label=self.label.clone() on clicked=|_| Message::ToggleInput />
                <Revealer reveal_child=self.input_open transition_type=RevealerTransitionType::SlideLeft>
                    <Entry on activate=|e| Message::Enter(e.get_text().to_string()) />
                </Revealer>
            </Box>
        }
    }
}
