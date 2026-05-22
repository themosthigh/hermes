use relm4_icons_build;

#[path = "src/icons.rs"]
mod icons;

fn main() {
    relm4_icons_build::bundle_icons(
        // Name of the generated file inside OUT_DIR
        "icon_names.rs",
        // Optional App ID
        Some("dev.themosthigh.hermes"),
        None::<&str>,
        None::<&str>,
        // Declare the explicit icons you want compiled into the binary
        icons::ICONS,
    );
}
