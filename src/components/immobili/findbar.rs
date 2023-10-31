#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use crate::components::commons::multiselect::{MultiSelectData, MultiSelectDataItem};

use super::super::commons::multiselect::MultiSelect;

pub fn FindBarImmobili(cx: Scope) -> Element {
    let mut x:Vec<MultiSelectDataItem> = Vec::new();
    let item1:MultiSelectDataItem = MultiSelectDataItem {key:1, value_return:("1".to_string()), value_display:("Casa singola".to_string())};
    let item2:MultiSelectDataItem = MultiSelectDataItem {key:2, value_return:("2".to_string()), value_display:("Appartamento".to_string())};
    let item3:MultiSelectDataItem = MultiSelectDataItem {key:3, value_return:("3".to_string()), value_display:("Villetta".to_string())};
    x.push(item1);
    x.push(item2);
    x.push(item3);
    let y:MultiSelectData = MultiSelectData { label_name: ("Tipi immobili".to_string()), items: (x)};
    cx.render(rsx!(
        div {
            class:"rounded-lg border-2 bg-gray-100 mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
            div {
                class:"flex space-x-4",
                MultiSelect{data:y}
            }
        }
    ))
}