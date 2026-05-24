use adw::glib;
use relm4::gtk;
use relm4::gtk::prelude::*;

pub fn register_shortcut<W, F>(root: &W, _action_name: &str, key_combo: &str, callback: F)
where
    W: IsA<gtk::Widget>,
    F: Fn() + 'static,
{
    let trigger = gtk::ShortcutTrigger::parse_string(key_combo).unwrap();

    // Wrap your callback directly into a GTK action closure
    let action = gtk::CallbackAction::new(move |_, _| {
        callback();
        glib::Propagation::Stop // Keeps other widgets from stealing the event
    });

    let shortcut = gtk::Shortcut::builder()
        .trigger(&trigger)
        .action(&action)
        .build();

    let shortcut_controller = gtk::ShortcutController::new();
    shortcut_controller.set_scope(gtk::ShortcutScope::Global);
    shortcut_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
    shortcut_controller.add_shortcut(shortcut);

    root.add_controller(shortcut_controller);
}
