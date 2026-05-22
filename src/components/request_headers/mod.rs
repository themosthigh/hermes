use relm4::gtk::prelude::*;
use relm4::prelude::*;

mod request_header_input;
use request_header_input as header_input;

use crate::request::RequestHeader;

#[derive(Debug)]
pub struct Model {
    headers: Vec<RequestHeader>,
    header_input_widgets: FactoryVecDeque<header_input::Model>,
}

#[derive(Debug)]
pub enum Msg {
    HeadersChanged(Vec<RequestHeader>),
    HeaderChanged(usize, RequestHeader),
    AddHeader,
    SendHeadersToParent,
}

#[derive(Debug, Clone)]
pub enum Output {
    EmitHeadersChanged(Vec<RequestHeader>),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Vec<RequestHeader>;
    type Input = Msg;
    type Output = Output;

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,
            add_css_class: "card",

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 10,
                set_hexpand: true,
                set_vexpand: true,

                #[local_ref]
                headers_list_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_hexpand: true,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_all: 20,

                    gtk::Button {
                        set_label: "Add",
                        set_hexpand: true,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        add_css_class: "suggested-action",
                        add_css_class: "large",
                        add_css_class: "pill",
                        set_width_request: 120,
                        connect_clicked => Msg::AddHeader,
                    },
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut header_input_widgets = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                header_input::Output::EmitHeaderChanged(index, header) => {
                    Msg::HeaderChanged(index, header)
                }
            });

        {
            // Initialize header input widgets
            let mut guard = header_input_widgets.guard();
            guard.clear();
            for header in &init {
                guard.push_back(header.clone());
            }
        };

        let model = Model {
            headers: init,
            header_input_widgets,
        };

        let headers_list_box = model.header_input_widgets.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::HeaderChanged(index, header) => {
                self.headers[index] = header;
                let _ = sender.output(Output::EmitHeadersChanged(self.headers.clone()));
            }
            Msg::HeadersChanged(headers) => {
                self.headers = headers;
                let mut guard = self.header_input_widgets.guard();
                guard.clear();

                for header in &self.headers {
                    guard.push_back(header.clone());
                }
            }
            Msg::AddHeader => {
                self.headers.push(RequestHeader {
                    name: String::new(),
                    value: String::new(),
                    enabled: true,
                });
                sender.input(Msg::HeadersChanged(self.headers.clone()));
            }
            Msg::SendHeadersToParent => {
                let _ = sender.output(Output::EmitHeadersChanged(self.headers.clone()));
            }
        }
    }
}
