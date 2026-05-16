use relm4::{gtk, gtk::prelude::*, prelude::*};
use sourceview5::prelude::*;

use crate::utils::sourceview::init_source_buffer;

#[derive(Debug)]
pub struct Model {
    response: String,
    source_buffer: sourceview5::Buffer,
}

#[derive(Debug)]
pub enum Msg {
    Update(String),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = String;
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
                set_highlight_current_line : true,
                set_pixels_above_lines: 2,
                set_pixels_below_lines: 2,
                add_css_class: "source-view",
            },
        }
    }

    fn init(
        response: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            response,
            source_buffer: init_source_buffer(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::Update(response) => {
                self.response = response;
                self.source_buffer.set_text(&self.response);
            }
        }
    }
}
