use std::sync::Arc;

use relm4::{gtk::prelude::*, prelude::*};

use crate::{request::RequestState, utils::store::LocalStore};

mod import_button;
mod request_option_dropdown;

#[derive(Debug)]
pub struct Model {
    request_store: Arc<LocalStore<RequestState>>,
    request_option_dropdown_widget: Controller<request_option_dropdown::Model>,
    import_file_widget: Controller<import_button::Model>,
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

           pack_end = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                append = model.import_file_widget.widget(),
                append = model.request_option_dropdown_widget.widget(),

            },
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
            request_store: init.request_store.clone(),
            label_widget: label,
            request_option_dropdown_widget: request_option_dropdown::Model::builder()
                .launch(init.request_store.clone())
                .detach(),
            import_file_widget: import_button::Model::builder()
                .launch(init.request_store)
                .detach(),
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
