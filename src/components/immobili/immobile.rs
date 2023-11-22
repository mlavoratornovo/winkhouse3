#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use super::super::navbar::Navbar;

pub fn Immobile(cx: Scope) -> Element {

    cx.render(rsx!(
        Navbar(&cx, 6)
        "Dettaglio immobile"
    ))
}