#[allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        style { include_str!("assets/app.scss") }
        div { class: "app_background",
            div {
                "hi"
            }
        }
    ))
}

fn app_settings(title: &str) -> Config {
    let config = Config::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(
            800.0, 550.0
        )
    );

    config.with_window(window)
}

/**
 * Initializes the GUI with the given config (Which is platform specific), and the index.
 */
pub fn init_gui() {
    dioxus_desktop::launch_cfg(
        app, 
        app_settings("Nebula Launcher")
    )
}