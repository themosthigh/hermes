use adw::prelude::*;
use relm4::prelude::*;

use crate::{
    components::{
        request_bar, request_body, request_headers, request_params, response_headers,
        response_preview,
    },
    request::{RequestHeader, RequestState, ResponseState},
    utils::network::{format_json, send_request},
};

#[derive(Debug)]
pub struct Model {
    request_bar_widget: Controller<request_bar::Model>,
    request_headers_widget: Controller<request_headers::Model>,
    request_params_widget: Controller<request_params::Model>,
    request_body_widget: Controller<request_body::Model>,
    response_preview_widget: Controller<response_preview::Model>,
    response_headers_widget: Controller<response_headers::Model>,
    request: RequestState,
    response: Option<ResponseState>,
    response_preview: String,
}

#[derive(Debug)]
pub enum Msg {
    SendRequest,
    UpdateRequest(RequestState),
    UpdateRequestFromBar(request_bar::Model),
    UpdateRequestBody(String),
    UpdateResponsePreview(String),
    UpdateResponse(Option<ResponseState>),
    UpdateRequestHeaders(Vec<RequestHeader>),
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

                #[name="breakpoint_bin"]
                adw::BreakpointBin {
                    set_width_request: 300,
                    set_height_request: 300,
                    // Request and response
                    #[wrap(Some)]
                    #[name="request_and_response"]
                    set_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_homogeneous: true,
                        set_spacing: 5,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_spacing: 5,

                            adw::InlineViewSwitcher {
                                set_stack: Some(&request_stack),
                            },

                            #[name="request_stack"]
                            adw::ViewStack {
                                set_vexpand: true,
                            }
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_spacing: 5,

                            adw::InlineViewSwitcher {
                                set_stack: Some(&response_stack),
                            },

                            #[name="response_stack"]
                            adw::ViewStack {
                                set_vexpand: true,
                            },
                        }
                    },
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

        let request_headers_widget = request_headers::Model::builder()
            .launch(request.headers.clone())
            .forward(sender.input_sender(), |output| match output {
                request_headers::Output::EmitHeadersChanged(headers) => {
                    Msg::UpdateRequestHeaders(headers)
                }
            });

        let res = response_preview::Model::builder()
            .launch(String::from("Response preview section"))
            .detach();

        let res_headers = response_headers::Model::builder()
            .launch(Vec::new())
            .detach();

        // Initialise Main Content Model
        let model = Model {
            request_bar_widget: req,
            request_body_widget: req_body,
            request_headers_widget,
            request_params_widget: request_params::Model::builder().launch(()).detach(),
            response_headers_widget: res_headers,
            response_preview_widget: res,
            response_preview: String::new(),
            request: request.clone(),
            response: None,
        };

        let widgets = view_output!();

        // Add Header and Body tabs
        widgets.request_stack.add_titled(
            model.request_headers_widget.widget(),
            Some("request_headers"),
            "Headers",
        );
        widgets.request_stack.add_titled(
            model.request_body_widget.widget(),
            Some("request_body"),
            "Body",
        );

        widgets.request_stack.add_titled(
            model.request_params_widget.widget(),
            Some("request_params"),
            "Params",
        );

        // Add Response preview tabs
        widgets.response_stack.add_titled(
            model.response_preview_widget.widget(),
            Some("response_preview"),
            "Preview",
        );

        widgets.response_stack.add_titled(
            model.response_headers_widget.widget(),
            Some("response_headers"),
            "Headers",
        );

        // Add layout breakpoint
        let breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
            adw::BreakpointConditionLengthType::MaxWidth,
            750.0,
            adw::LengthUnit::Px,
        ));
        breakpoint.add_setter(
            &widgets.request_and_response,
            "orientation",
            Some(&gtk::Orientation::Vertical.to_value()),
        );
        widgets.breakpoint_bin.add_breakpoint(breakpoint);

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
            Msg::UpdateRequestHeaders(headers) => {
                self.request.headers = headers.clone();
            }
            Msg::UpdateRequestBody(body) => {
                self.request.body = body;
            }
            Msg::SendRequest => {
                // Show loading state
                sender.input(Msg::UpdateResponsePreview(String::from("Loading...")));
                let req = self.request.clone();

                // Send request
                sender
                    .clone()
                    .command(move |_sender, _shutdown| async move {
                        match send_request(req).await {
                            Ok(response) => {
                                let _ = sender.input(Msg::UpdateResponse(Some(response)));
                            }
                            Err(error) => {
                                let _ = sender
                                    .input(Msg::UpdateResponsePreview(format_json(error).unwrap()));
                            }
                        };
                    });
            }
            Msg::UpdateResponsePreview(text) => {
                self.response_preview = text;
                let _ = self
                    .response_preview_widget
                    .sender()
                    .send(response_preview::Msg::Update(
                        self.response_preview.to_string(),
                    ));
            }
            Msg::UpdateResponse(response) => {
                self.response = response;

                // update preview
                let preview = match &self.response {
                    Some(response) => &response.body,
                    None => &String::from("No response"),
                };
                let formatted_preview = format_json(preview.to_string());
                match formatted_preview {
                    Ok(formatted_preview) => {
                        sender.input(Msg::UpdateResponsePreview(formatted_preview));
                    }
                    Err(error) => {
                        println!("Formatting error: {}", error);
                        sender.input(Msg::UpdateResponsePreview(preview.to_string()));
                    }
                }
                // update headers
                let headers = match &self.response {
                    Some(response) => response.headers.clone(),
                    None => Vec::new(),
                };
                let _ = self
                    .response_headers_widget
                    .sender()
                    .send(response_headers::Msg::HeadersChanged(headers));
            }
        }
    }
}
