use std::sync::Arc;

use adw::prelude::*;
use relm4::prelude::*;
use tokio::runtime::Handle;

use crate::{
    components::{
        app_header, request_bar, request_body, request_headers, request_params, response_headers,
        response_preview,
    },
    request::{RequestState, ResponseState},
    utils::{network::send_request, store::LocalStore},
};

#[derive(Debug)]
pub struct Model {
    app_header_widget: Controller<app_header::Model>,
    request_bar_widget: Controller<request_bar::Model>,
    request_headers_widget: Controller<request_headers::Model>,
    request_params_widget: Controller<request_params::Model>,
    request_body_widget: Controller<request_body::Model>,
    response_preview_widget: Controller<response_preview::Model>,
    response_headers_widget: Controller<response_headers::Model>,

    // stores
    request_store: Arc<LocalStore<RequestState>>,
    response_store: Arc<LocalStore<ResponseState>>,
}

#[derive(Debug)]
pub enum Msg {
    SendRequest,
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        adw::ToolbarView {
            add_top_bar = model.app_header_widget.widget(),

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
        let request_store = Arc::new(LocalStore::new(request.clone()));
        let response_store = Arc::new(LocalStore::new(ResponseState::default()));

        let req = request_bar::Model::builder()
            .launch(request_store.clone())
            .forward(sender.input_sender(), |req_output| match req_output {
                request_bar::Output::EmitSend => Msg::SendRequest,
            });

        let req_body = request_body::Model::builder()
            .launch(request_store.clone())
            .detach();

        let request_headers_widget = request_headers::Model::builder()
            .launch(request.headers.clone())
            .detach();

        let res = response_preview::Model::builder()
            .launch(response_store.clone())
            .detach();

        let res_headers = response_headers::Model::builder()
            .launch(response_store.clone())
            .detach();

        // Initialise Main Content Model
        let model = Model {
            app_header_widget: app_header::Model::builder()
                .launch(app_header::Init {
                    request_store: request_store.clone(),
                })
                .detach(),
            request_bar_widget: req,
            request_body_widget: req_body,
            request_headers_widget,
            request_params_widget: request_params::Model::builder().launch(()).detach(),
            response_headers_widget: res_headers,
            response_preview_widget: res,
            // stores
            request_store: request_store.clone(),
            response_store: response_store.clone(),
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

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::SendRequest => {
                // Show loading state
                let req = self.request_store.get_current();

                // Send request

                let handle = Handle::current();
                let result = handle.block_on(async { send_request(req).await });
                self.response_store.update(|state| {
                    match result {
                        Ok(res) => {
                            state.status_code = res.status_code;
                            state.headers = res.headers;
                            state.body = res.body;
                        }
                        Err(error) => {
                            state.headers = vec![];
                            state.status_code = 0;
                            state.body = String::from(format!("{}", error))
                        }
                    };
                });
            }
        }
    }
}
