use std::sync::Arc;

use adw::prelude::AdwDialogExt;
use relm4::{
    actions::{ActionGroupName, RelmAction, RelmActionGroup},
    gtk::prelude::*,
    prelude::*,
};

use crate::{
    AboutAction, QuitAction, WindowActionGroup, components::dialog::curl_import_dialog, icon_names,
    request::RequestState, utils::store::LocalStore,
};

#[derive(Debug)]
pub struct Model {
    _curl_import_dialog_ctrlr: Controller<curl_import_dialog::Model>,
}

pub type Init = Arc<LocalStore<RequestState>>;

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = ();
    type Output = ();

    menu! {
        request_menu: {
            section! {
                import_curl => OpenImportCurlDialog,
            },
            section! {
                about => AboutAction
            },
            section! {
                quit => QuitAction
            }
        }
    }

    view! {
        #[root]
        main_box = gtk::Box {
            gtk::MenuButton {
                set_icon_name: icon_names::shipped::MENU,
                set_menu_model: Some(&request_menu)
            }
        },

        popover_child = gtk::Spinner {
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let import_curl = "Import cURL";
        let about = "About Hermes";
        let quit = "Quit";

        let dialog_controller = curl_import_dialog::Model::builder()
            .launch(init.clone())
            .detach();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let root_clone = root.clone();
        let dialog_widget = dialog_controller.widget().clone();
        let import_curl_action = {
            RelmAction::<OpenImportCurlDialog>::new_stateless(move |_| {
                dialog_widget.present(Some(&root_clone));
            })
        };

        actions.add_action(import_curl_action);

        root.insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        let widgets = view_output!();
        let model = Model {
            _curl_import_dialog_ctrlr: dialog_controller,
        };

        ComponentParts { model, widgets }
    }
}

relm4::new_stateless_action!(OpenImportCurlDialog, WindowActionGroup, "open-import-curl");
