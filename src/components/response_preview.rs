use std::sync::Arc;

use relm4::{gtk, gtk::prelude::*, prelude::*};
use sourceview5::prelude::*;

use crate::{
    request::ResponseState,
    utils::{sourceview::init_source_buffer, store::LocalStore},
};

#[derive(Debug)]
pub struct Model {
    response_store: Init,
    source_buffer: sourceview5::Buffer,
}

#[derive(Debug)]
pub enum Msg {
    Update(String),
}

pub type Init = Arc<LocalStore<ResponseState>>;

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Msg;
    type Output = ();

    view! {

        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,

            #[name="text_view"]
            sourceview5::View {
                set_buffer: Some(&model.source_buffer),
                set_editable: false,
                set_show_line_numbers: true,
                set_highlight_current_line : false,
                set_pixels_above_lines: 2,
                set_pixels_below_lines: 2,
                set_monospace: true,
                add_css_class: "source-view",
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            response_store: init,
            source_buffer: init_source_buffer(None),
        };
        let widgets = view_output!();

        model
            .response_store
            .connect(&sender, |response_state| Msg::Update(response_state.body));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Update(response) => {
                self.source_buffer.set_text(response.as_str());
            }
        }
    }
}
