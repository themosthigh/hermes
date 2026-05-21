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
            /*
            pack_start = &gtk::Button {
                set_label: "Import",
                connect_clicked => move |_| {
                    println!("Importing")
                }
            }
            */
        }
    }

    fn init(
        init: Self::Init,
        _window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let label = gtk::Label::new(Some("Hermes"));
        label.set_ellipsize(gtk::pango::EllipsizeMode::Middle);

        let model = Model {
            request_store: init.request_store,
            label_widget: label,
        };

        model
            .request_store
            .connect(&sender, |state| Msg::UpdateTitle(state.url));

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
