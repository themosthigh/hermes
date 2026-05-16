use adw::glib::clone;
use sourceview5::prelude::*;

pub fn init_source_buffer() -> sourceview5::Buffer {
    let buffer = sourceview5::Buffer::new(None);
    let lang_manager = sourceview5::LanguageManager::default();
    if let Some(rust_lang) = lang_manager.language("json") {
        buffer.set_language(Some(&rust_lang));
    }
    update_buffer_theme(&buffer);

    // I genuinely don't understand how this works
    let buffer_clone = buffer.clone();
    let style_manager = adw::StyleManager::default();
    style_manager.connect_dark_notify(clone!(move |_style_manager| {
        update_buffer_theme(&buffer_clone);
    }));

    buffer
}

// Helper function to update the theme scheme
fn update_buffer_theme(buffer: &sourceview5::Buffer) {
    let style_manager = adw::StyleManager::default();
    let scheme_manager = sourceview5::StyleSchemeManager::default();

    // Choose "Adwaita-dark" if system is in dark mode, otherwise "Adwaita"
    let scheme_name = if style_manager.is_dark() {
        "Adwaita-dark"
    } else {
        "Adwaita"
    };

    if let Some(theme_scheme) = scheme_manager.scheme(scheme_name) {
        buffer.set_style_scheme(Some(&theme_scheme));
    }
}
