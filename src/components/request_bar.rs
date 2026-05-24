use std::sync::Arc;

use relm4::{gtk, gtk::prelude::*, prelude::*};

use crate::{
    request::{METHODS, RequestState},
    utils::{shortcut::register_shortcut, store::LocalStore},
};

pub type Init = Arc<LocalStore<RequestState>>;

#[derive(Debug, Clone)]
pub struct Model {
    pub url: String,
    pub url_buffer: gtk::EntryBuffer,
    pub method: String,
    pub request_store: Init,
}

#[derive(Debug)]
pub enum Msg {
    Send,
    UrlChanged(String),
    MethodChanged(String),
    StoreValueChanged(RequestState),
}

#[derive(Debug)]
pub struct OutputData {
    pub url: String,
    pub method: String,
}

#[derive(Debug)]
pub enum Output {
    EmitSend,
}

const THIS_SOURCE: &str = "request-bar";

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Msg;
    type Output = Output;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            add_css_class : "response-bar",

            #[name="method_dropdown"]
            gtk::DropDown {
                add_css_class: "method-dropdown",
                set_model: Some(gtk::StringList::new(METHODS).upcast_ref::<gtk::gio::ListModel>()),
                // Compute index from static slice
                #[watch]
                set_selected: METHODS.iter().position(|&method| method == &model.method).unwrap() as u32,

                // Update method from dropdown
                connect_selected_notify[sender] => move |dropdown| {
                    let index = dropdown.selected() as usize;
                    sender.input(Msg::MethodChanged(METHODS[index].to_string()));
                },
            },

            #[name="url_entry"]
            gtk::Entry {
                set_placeholder_text: Some("Enter URL ..."),
                set_buffer: &model.url_buffer,
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let text = entry.text().to_string();
                        sender.input(Msg::UrlChanged(text));
                },
                connect_activate[sender] => move |_entry| {
                    let _ = sender.output(Output::EmitSend);
                },
                inline_css: "border-radius: 0px"
            },

            gtk::Button {
                set_label: "Send",
                connect_clicked => Msg::Send,
                add_css_class: "send-button",
            }
        },
    }

    fn post_view() {
        if model.request_store.get_source() != String::from(THIS_SOURCE)
            && model.request_store.get_current().url != model.url
        {
            widgets
                .url_entry
                .set_text(model.request_store.get_current().url.as_str());
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // Initialise Model
        let model = Model {
            url: String::new(),
            url_buffer: gtk::EntryBuffer::new(Some(init.clone().get_current().url)),
            method: String::from("GET"),
            request_store: init.clone(),
        };

        let widgets = view_output!();

        widgets.url_entry.set_text(init.get_current().url.as_str());

        // Focus entry on ctrl+l
        {
            let entry_clone = widgets.url_entry.clone();
            let root = root.clone();
            register_shortcut(&root, "focus_entry", "<Primary>l", move || {
                entry_clone.grab_focus();
            });
        }

        // Send request on ctrl+r
        {
            let sender = sender.clone();
            let root = root.clone();
            register_shortcut(&root, "send_request", "<Primary>r", move || {
                println!("Running");
                let _ = sender.output(Output::EmitSend);
            });
        }

        // listen for store updates
        let sender = sender.clone();
        init.connect(&sender, Msg::StoreValueChanged);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Send => {
                let _ = sender.output(Output::EmitSend);
            }
            Msg::UrlChanged(url) => {
                if url != self.url {
                    self.request_store
                        .update_source(String::from(THIS_SOURCE))
                        .update(|request_state| {
                            request_state.url = url;
                        });
                }
            }
            Msg::MethodChanged(method) => {
                if method != self.method {
                    self.request_store
                        .update_source(String::from(THIS_SOURCE))
                        .update(|request_state| {
                            request_state.method = method;
                        });
                }
            }
            Msg::StoreValueChanged(new_state) => {
                if self.request_store.get_source() != String::from(THIS_SOURCE) {
                    self.url = new_state.url.clone();
                    self.method = new_state.method;
                    self.url_buffer.set_text(new_state.url);
                }
            }
        }
    }
}
