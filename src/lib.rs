pub mod components;
pub mod request;
pub mod utils;

pub mod icon_names {
    use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
