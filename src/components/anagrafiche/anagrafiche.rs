#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};

pub fn Anagrafiche(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            "Anagrafiche"
        }
    ))
}