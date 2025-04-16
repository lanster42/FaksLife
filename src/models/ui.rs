use sauron::prelude::*;

struct App;

impl Component<()> for App {
    fn view(&self) -> Node<()> {
        div([], [text("Hello from Sauron!")])
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sauron::program::start(&App);
}
