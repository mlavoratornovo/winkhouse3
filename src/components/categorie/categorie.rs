#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use super::super::navbar::Navbar;

pub fn Categorie(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            Navbar(&cx, 4)            
        }
    ))
}
