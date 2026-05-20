use std::sync::Arc;

use relm4::prelude::*;

use crate::{request::RequestState, utils::store::LocalStore};

#[derive(Debug)]
pub struct Model {
    request_store: Arc<LocalStore<RequestState>>,
    label_widget: gtk::Label,
}

#[derive(Debug)]
pub struct Init {
    pub request_store: Arc<LocalStore<RequestState>>,
}

#[derive(Debug)]
pub enum Msg {
    UpdateTitle(String),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Msg;
    type Output = ();

    view! {
        adw::HeaderBar {
            set_title_widget = Some(&model.label_widget),
        }
    }

    fn init(
        init: Self::Init,
        _window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            request_store: init.request_store,
            label_widget: gtk::Label::new(Some("Hermes ++")),
        };

        let (channel_sender, channel_receiver) = relm4::channel::<RequestState>();
        model.request_store.subscribe(channel_sender);

        let input_sender = sender.input_sender().clone();
        relm4::spawn_local(async move {
            while let Some(request_state) = channel_receiver.recv().await {
                input_sender
                    .send(Msg::UpdateTitle(request_state.url))
                    .unwrap();
            }
        });

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Msg::UpdateTitle(title) => {
                self.label_widget
                    .set_text(if title.is_empty() { "Hermes" } else { &title });
            }
        }
    }
}
