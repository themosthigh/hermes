use std::sync::Arc;

use relm4::{adw::prelude::*, prelude::*};
use sourceview5::prelude::*;

use crate::{
    request::RequestState,
    utils::{
        sourceview::{SourceBufferOptions, init_source_buffer},
        store::LocalStore,
    },
};

#[derive(Debug)]
pub struct Model {
    request_store: Init,
    source_buffer: sourceview5::Buffer,
    curl_text: String,
}

#[derive(Debug)]
pub enum Msg {
    AttemptCurlConversion,
    UpdateCurlText(String),
    UpdateRequest(RequestState),
}

type Init = Arc<LocalStore<RequestState>>;

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Msg;
    type Output = ();

    view! {
        #[name="import_curl_dialog"]
        adw::Dialog {
            set_title: "Import curl",
            set_content_width: 600,
            set_content_height: 400,

            #[wrap(Some)]
            set_child =  &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,
                set_margin_all: 10,

                gtk::Label {
                    set_label: "Import curl Request"
                },

                sourceview5::View {
                    set_buffer: Some(&model.source_buffer),
                    set_editable: true,
                    set_vexpand: true,
                    set_show_line_numbers: true,
                    set_highlight_current_line : false,
                    set_pixels_above_lines: 2,
                    set_pixels_below_lines: 2,
                    set_tab_width: 4,
                    set_monospace: true,
                    add_css_class: "source-view",
                },

                gtk::Button {
                    set_label: "Import",
                    add_css_class: "large",
                    add_css_class: "suggested_action",
                    connect_clicked => Msg::AttemptCurlConversion
                }
            }
        }
    }

    fn init(init: Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let buffer = init_source_buffer(Some(SourceBufferOptions { language: "sh" }));

        let model = Model {
            request_store: init,
            source_buffer: buffer,
            curl_text: String::new(),
        };

        // Add on_change event to update buffer state
        let sender_clone = sender.clone();
        model.source_buffer.connect_changed(move |buffer| {
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let text = buffer.text(&start, &end, true).to_string();
            sender_clone.input(Msg::UpdateCurlText(text));
        });

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Msg::UpdateCurlText(text) => {
                self.curl_text = text;
            }

            Msg::AttemptCurlConversion => match RequestState::from_curl(&self.curl_text) {
                Ok(new_state) => {
                    sender.input(Msg::UpdateRequest(new_state));
                }
                Err(e) => {
                    println!("Eroor {}", e);
                }
            },

            Msg::UpdateRequest(new_state) => {
                self.request_store
                    .update_source(String::from("curl-input"))
                    .update(|state| {
                        state.url = new_state.url;
                        state.method = new_state.method;
                        state.body = new_state.body;
                        state.headers = new_state.headers;
                    });
            }
        }
    }
}
