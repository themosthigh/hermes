pub mod components;
pub mod request;
pub mod utils;

pub mod icon_names {
    use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

// Actions
relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(pub AboutAction, WindowActionGroup, "show-about-dialog");
relm4::new_stateless_action!(pub QuitAction, WindowActionGroup, "quit-hermes");
