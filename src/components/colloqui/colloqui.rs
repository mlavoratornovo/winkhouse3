#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use super::findbar::FindBarColloqui;
use super::super::navbar::Navbar;

pub fn Colloqui(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            Navbar(&cx, 3)
            FindBarColloqui(cx)
        }
    ))
}