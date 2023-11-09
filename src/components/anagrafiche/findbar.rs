#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use crate::components::commons::multiselect::{MultiSelectData, MultiSelectDataItem};

use super::super::commons::multiselect::MultiSelect;

pub fn FindBarAnagrafiche(cx: Scope) -> Element {

    let mut tipologieAnagrafiche:Vec<MultiSelectDataItem> = Vec::new();
    let item1:MultiSelectDataItem = MultiSelectDataItem {key:1, value_return:("1".to_string()), value_display:("Propietario".to_string())};
    let item2:MultiSelectDataItem = MultiSelectDataItem {key:2, value_return:("2".to_string()), value_display:("Affittuario".to_string())};
    let item3:MultiSelectDataItem = MultiSelectDataItem {key:3, value_return:("3".to_string()), value_display:("Azienda edile".to_string())};
    tipologieAnagrafiche.push(item1);
    tipologieAnagrafiche.push(item2);
    tipologieAnagrafiche.push(item3);
    let x:MultiSelectData = MultiSelectData { key: 1, label_name: ("Tipi anagrafiche".to_string()), items: (tipologieAnagrafiche)};

    cx.render(rsx!(
        div {
            class:"shadow-xl rounded-lg border-2 bg-gray-100 mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
            div {
                class:"flex space-x-4 relative",
                MultiSelect{data:x}
            }
        }
    ))
}