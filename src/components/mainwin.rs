#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}, html::{text, svg}};
use super::navbar::Navbar;

pub fn Mainwin(cx: Scope) -> Element {

    cx.render(rsx!(
        Navbar(cx)
    ))
}