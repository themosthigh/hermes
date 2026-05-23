use std::sync::Arc;

use relm4::gtk::prelude::*;
use relm4::prelude::*;
use sourceview5::prelude::*;

use crate::request::RequestState;
use crate::utils::sourceview::{SourceBufferOptions, init_source_buffer};
use crate::utils::store::LocalStore;

#[derive(Debug)]
pub struct Model {
    request_store: Init,
    source_buffer: sourceview5::Buffer,
    current_buffer_text: String,
}

#[derive(Debug)]
pub enum Msg {
    RequestBodyChanged(String),
    RequestStateChanged(RequestState),
}

#[derive(Debug)]
pub enum Output {
    UpdateRequestBody(String),
}

pub type Init = Arc<LocalStore<RequestState>>;

const THIS_SOURCE: &str = "request-body";

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Msg;
    type Output = Output;

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,

            sourceview5::View {
                set_buffer: Some(&model.source_buffer),
                set_editable: true,
                set_show_line_numbers: true,
                set_highlight_current_line : false,
                set_pixels_above_lines: 2,
                set_pixels_below_lines: 2,
                set_tab_width: 4,
                set_monospace: true,
                add_css_class: "source-view",
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            request_store: init,
            source_buffer: init_source_buffer(Some(SourceBufferOptions { language: "json" })),
            current_buffer_text: String::new(),
        };
        model
            .source_buffer
            .set_text(&model.request_store.get_current().body);

        // Add on_change event to update buffer state
        {
            let sender_clone = sender.clone();
            model.source_buffer.connect_changed(move |buffer| {
                let start = buffer.start_iter();
                let end = buffer.end_iter();
                let text = buffer.text(&start, &end, true).to_string();
                sender_clone.input(Msg::RequestBodyChanged(text));
            });
        }

        // Listen for external updates
        {
            let sender_clone = sender.clone();
            model
                .request_store
                .connect(&sender_clone, Msg::RequestStateChanged);
        }

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::RequestBodyChanged(request_body) => {
                self.current_buffer_text = request_body.clone();
                self.request_store
                    .update_source(String::from(THIS_SOURCE))
                    .update(|request_state| {
                        request_state.body = request_body;
                    });
            }

            Msg::RequestStateChanged(new_state) => {
                if self.request_store.get_source() != String::from(THIS_SOURCE)
                    && self.current_buffer_text != new_state.body
                {
                    self.source_buffer.set_text(&new_state.body);
                }
            }
        }
    }
}
