use dioxus::prelude::*;

#[derive(Props)]
pub struct SwitchProps<'a> {
    pub enabled: bool,
    pub on_click: EventHandler<'a, ()>
}

#[allow(non_snake_case)]
pub fn AstralSwitch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let enabled = use_state(cx, || true);

    render!(
        div {

        }
    )
}