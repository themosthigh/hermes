use relm4::RelmApp;

mod app;
use hermes::icon_names;

const STYLES_CSS: &str = include_str!("./styles.css");

fn main() {
    let app = RelmApp::new("dev.themosthigh.hermes");

    // Initialise css
    relm4::set_global_css(STYLES_CSS);

    // Include icons
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    // Run app
    app.run::<app::Model>(());
}
