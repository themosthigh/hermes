use std::sync::Arc;

use relm4::gtk::prelude::*;
use relm4::prelude::*;
use sourceview5::prelude::*;

use crate::request::RequestState;
use crate::utils::sourceview::init_source_buffer;
use crate::utils::store::LocalStore;

#[derive(Debug)]
pub struct Model {
    request_store: Init,
    source_buffer: sourceview5::Buffer,
}

#[derive(Debug)]
pub enum Msg {
    RequestBodyChanged(String),
}

#[derive(Debug)]
pub enum Output {
    UpdateRequestBody(String),
}

pub type Init = Arc<LocalStore<RequestState>>;

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
            source_buffer: init_source_buffer(),
        };
        model
            .source_buffer
            .set_text(&model.request_store.get_current().body);

        // Add on_change event to update buffer state
        model.source_buffer.connect_changed(move |buffer| {
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let text = buffer.text(&start, &end, true).to_string();
            sender.input(Msg::RequestBodyChanged(text));
        });

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::RequestBodyChanged(request_body) => {
                self.request_store.update(|request_state| {
                    request_state.body = request_body;
                });
            }
        }
    }
}
