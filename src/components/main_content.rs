use adw::prelude::*;
use relm4::prelude::*;

use crate::components::request_bar;
use crate::components::request_body;
use crate::components::response_preview;
use crate::request::RequestState;
use crate::utils::network::send_request;

#[derive(Debug)]
pub struct Model {
    request_bar_widget: Controller<request_bar::Model>,
    request_body_widget: Controller<request_body::Model>,
    response_preview_widget: Controller<response_preview::Model>,
    request: RequestState,
    response_preview: String,
}

#[derive(Debug)]
pub enum Msg {
    SendRequest,
    UpdateRequest(RequestState),
    UpdateRequestFromBar(request_bar::Model),
    UpdateRequestBody(String),
    UpdateResponse(String),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar{
                set_css_classes: &["flat"],
                set_hexpand: true,
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: true,
                set_title_widget = Some(&gtk::Label::new(Some("Hermes"))),
            },

            #[wrap(Some)]
                set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_spacing: 5,
                set_margin_all: 5,
                // Request bar
                append = model.request_bar_widget.widget(),

                // Request and response
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_homogeneous: true,
                    set_spacing: 5,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,
                        // Response preview
                        append = model.request_body_widget.widget(),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,
                        // Request body
                        append = model.response_preview_widget.widget(),
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let request = RequestState::default();

        let req = request_bar::Model::builder()
            .launch(request_bar::Model {
                url: request.url.clone(),
                method: request.method.clone(),
            })
            .forward(sender.input_sender(), |req_output| match req_output {
                // get request from request bar
                request_bar::Output::Send => Msg::SendRequest,
                request_bar::Output::UpdateRequestFromBar(req) => Msg::UpdateRequestFromBar(req),
            });

        let req_body = request_body::Model::builder()
            .launch(request_body::Init {
                request_body: String::from("{}"),
            })
            .forward(
                sender.input_sender(),
                |req_body_output| match req_body_output {
                    request_body::Output::UpdateRequestBody(body) => Msg::UpdateRequestBody(body),
                },
            );

        let res = response_preview::Model::builder()
            .launch(String::from("Response preview section"))
            .detach();

        let model = Model {
            request_bar_widget: req,
            request_body_widget: req_body,
            response_preview_widget: res,
            response_preview: String::from("Response preview section"),
            request,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::UpdateRequest(request) => {
                self.request = request;
            }
            Msg::UpdateRequestFromBar(req_bar_model) => {
                self.request.url = req_bar_model.url;
                self.request.method = req_bar_model.method;
            }
            Msg::UpdateRequestBody(body) => {
                self.request.body = body;
            }
            Msg::SendRequest => {
                // Show loading state
                sender.input(Msg::UpdateResponse(String::from("Loading...")));
                let req = self.request.clone();

                // Send request
                sender
                    .clone()
                    .command(move |_sender, _shutdown| async move {
                        match send_request(req).await {
                            Ok(response) => {
                                let _ = sender.input(Msg::UpdateResponse(response));
                            }
                            Err(error) => {
                                let _ = sender.input(Msg::UpdateResponse(error));
                            }
                        };
                    });
            }
            Msg::UpdateResponse(response) => {
                self.response_preview = response;
                let _ = self
                    .response_preview_widget
                    .sender()
                    .send(response_preview::Msg::Update(
                        self.response_preview.to_string(),
                    ));
            }
        }
    }
}
