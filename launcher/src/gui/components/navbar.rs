use dioxus::prelude::*;

pub fn navbar(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "nav-background"
        }
    ))
}