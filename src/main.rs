use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        filter: "warn,bevy_portal=debug".into(),
        level: Level::DEBUG,
    });

    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: Level::WARN,
    });

    app.add_plugins(DefaultPlugins).run();
}
