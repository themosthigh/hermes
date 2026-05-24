use std::{path::PathBuf, sync::Arc};

use relm4::{gtk::prelude::*, prelude::*};

use relm4_components::open_dialog::{OpenDialog, OpenDialogMsg, OpenDialogSettings};

use crate::{icon_names, request::RequestState, utils::store::LocalStore};

const THIS_SOURCE: &str = "import-http-dialog";

#[derive(Debug)]
pub struct Model {
    request_store: Init,
    open_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
pub enum Msg {
    OpenImportDialog,
    OpenResponse(PathBuf),
    Ignore,
}

pub type Init = Arc<LocalStore<RequestState>>;

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Msg;
    type Output = ();

    view! {
         gtk::Button {
            set_icon_name: icon_names::shipped::CODE,
            connect_clicked => Msg::OpenImportDialog
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                relm4_components::open_dialog::OpenDialogResponse::Accept(path) => {
                    Msg::OpenResponse(path)
                }
                relm4_components::open_dialog::OpenDialogResponse::Cancel => Msg::Ignore,
            });

        let model = Model {
            request_store: init,
            open_dialog,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Msg::OpenImportDialog => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            Msg::OpenResponse(path_buf) => match std::fs::read_to_string(&path_buf) {
                Ok(content) => {
                    println!("Contents {content}");
                    let parsed = RequestState::from_http(content).unwrap();
                    self.request_store
                        .update_source(String::from(THIS_SOURCE))
                        .update(|state| {
                            state.url = parsed.url;
                            state.method = parsed.method;
                            state.headers = parsed.headers;
                            state.body = parsed.body;
                        });
                }
                Err(e) => {
                    println!("Error reading file {e}");
                }
            },
            Msg::Ignore => {}
        }
    }
}
