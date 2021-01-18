use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
struct Model {
    msg: String,
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    UpdateLabel,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            },
            Message::UpdateLabel => {
                self.msg = "Hello, world!".to_owned();
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("com.example.rustuivgtk"), ApplicationFlags::empty())>
                <Window border_width=20 on destroy=|_| Message::Exit>
                    <HeaderBar title="Hello VGTK!" show_close_button=true />
                        <Box spacing=10 orientation=Orientation::Vertical >
                            <Label label=self.msg.clone() />
                            <Button label="Click Me!" on clicked=|_| Message::UpdateLabel />
                        </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    std::process::exit(run::<Model>());
}

