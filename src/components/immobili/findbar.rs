#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use crate::components::commons::multiselect::{MultiSelectData, MultiSelectDataItem};

use super::super::commons::multiselect::MultiSelect;

pub fn FindBarImmobili(cx: Scope) -> Element {

    let mut tipologieImmobili:Vec<MultiSelectDataItem> = Vec::new();
    let item1:MultiSelectDataItem = MultiSelectDataItem {key:1, value_return:("1".to_string()), value_display:("Casa singola".to_string())};
    let item2:MultiSelectDataItem = MultiSelectDataItem {key:2, value_return:("2".to_string()), value_display:("Appartamento".to_string())};
    let item3:MultiSelectDataItem = MultiSelectDataItem {key:3, value_return:("3".to_string()), value_display:("Villetta".to_string())};
    tipologieImmobili.push(item1);
    tipologieImmobili.push(item2);
    tipologieImmobili.push(item3);
    let x:MultiSelectData = MultiSelectData { key: 1, label_name: ("Tipi immobili".to_string()), items: (tipologieImmobili)};

    let mut riscaldamento:Vec<MultiSelectDataItem> = Vec::new();
    let item1:MultiSelectDataItem = MultiSelectDataItem {key:1, value_return:("1".to_string()), value_display:("Caldaia autonoma".to_string())};
    let item2:MultiSelectDataItem = MultiSelectDataItem {key:2, value_return:("2".to_string()), value_display:("Stufa pellet".to_string())};
    let item3:MultiSelectDataItem = MultiSelectDataItem {key:3, value_return:("3".to_string()), value_display:("Pompa di calore".to_string())};
    riscaldamento.push(item1);
    riscaldamento.push(item2);
    riscaldamento.push(item3);
    let y:MultiSelectData = MultiSelectData { key: 2, label_name: ("Riscaldamento".to_string()), items: (riscaldamento)};

    let mut classeenergetica:Vec<MultiSelectDataItem> = Vec::new();
    let item1:MultiSelectDataItem = MultiSelectDataItem {key:1, value_return:("1".to_string()), value_display:("A".to_string())};
    let item2:MultiSelectDataItem = MultiSelectDataItem {key:2, value_return:("2".to_string()), value_display:("B".to_string())};
    let item3:MultiSelectDataItem = MultiSelectDataItem {key:3, value_return:("3".to_string()), value_display:("C".to_string())};
    classeenergetica.push(item1);
    classeenergetica.push(item2);
    classeenergetica.push(item3);
    let z:MultiSelectData = MultiSelectData { key: 3, label_name: ("Classe energetica".to_string()), items: (classeenergetica)};

    let mut statoconservativo:Vec<MultiSelectDataItem> = Vec::new();
    let item1:MultiSelectDataItem = MultiSelectDataItem {key:1, value_return:("1".to_string()), value_display:("Nuovo".to_string())};
    let item2:MultiSelectDataItem = MultiSelectDataItem {key:2, value_return:("2".to_string()), value_display:("Da ristrutturare".to_string())};
    let item3:MultiSelectDataItem = MultiSelectDataItem {key:3, value_return:("3".to_string()), value_display:("Sfasciato".to_string())};
    statoconservativo.push(item1);
    statoconservativo.push(item2);
    statoconservativo.push(item3);
    let k:MultiSelectData = MultiSelectData {  key: 4, label_name: ("Stato conservativo".to_string()), items: (statoconservativo)};

    cx.render(rsx!(
        div {
            class:"shadow-xl rounded-lg border-2 bg-gray-100 mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
            div {
                class:"flex space-x-4 relative",
                MultiSelect{data:x}
                MultiSelect{data:y}
                MultiSelect{data:z}
                MultiSelect{data:k}
            }
        }
    ))
}