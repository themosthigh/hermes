use std::sync::Arc;

use relm4::{adw::prelude::*, prelude::*};

use crate::{request::ResponseState, utils::store::LocalStore};

mod header_list_item;

#[derive(Debug)]
pub struct Model {
    headers_list_widgets: FactoryVecDeque<header_list_item::Model>,
    response_store: Arc<LocalStore<ResponseState>>,
}

#[derive(Debug)]
pub enum Msg {
    HeadersChanged(Vec<(String, String)>),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Arc<LocalStore<ResponseState>>;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,
            add_css_class: "card",

            #[local_ref]
            headers_list_box -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let headers_list_widgets = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let model = Model {
            headers_list_widgets,
            response_store: init,
        };

        let headers_list_box = model.headers_list_widgets.widget();
        let widgets = view_output!();

        model.response_store.connect(&sender, |response_state| {
            Msg::HeadersChanged(response_state.headers)
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::HeadersChanged(headers) => {
                let mut guard = self.headers_list_widgets.guard();
                guard.clear();

                for (name, value) in headers {
                    guard.push_back(header_list_item::Model {
                        name: name.clone(),
                        value: value.clone(),
                    });
                }
            }
        }
    }
}
