#![allow(non_snake_case, unused)]
use dioxus::prelude::{*, GlobalAttributes};
use dioxus_fullstack::prelude::*;
mod components;
fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {    

    cx.render(rsx! {
        components::mainwin::Mainwin(&cx)
    })
}

