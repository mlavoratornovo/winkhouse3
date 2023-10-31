#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use super::findbar::FindBarImmobili;
use super::super::navbar::Navbar;
pub fn Immobili(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            Navbar(cx)
            FindBarImmobili(cx)
        }
    ))
}