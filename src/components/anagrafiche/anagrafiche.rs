#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use super::findbar::FindBarAnagrafiche;
use super::super::navbar::Navbar;

pub fn Anagrafiche(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            Navbar(&cx, 2)
            FindBarAnagrafiche(cx)
        }
    ))
}
