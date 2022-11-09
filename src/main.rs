use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};

fn main() {
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
