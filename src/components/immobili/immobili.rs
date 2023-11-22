#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use super::findbar::FindBarImmobili;
use super::super::navbar::Navbar;
use super::super::commons::tableview::TableView;
use crate::components::commons::tableview::{TableViewData, TableRow, PaginationInfo};
use crate::components::commons::winkentityenum::WinkEntity;
pub fn Immobili(cx: Scope) -> Element {

    let mut rows:Vec<TableRow> = Vec::new();

    let mut columns0:Vec<(String,String)> = Vec::new();
    columns0.push(("Via Cavour n.5".to_string(),"Roma".to_string()));
    columns0.push(("Mq. 150".to_string(),"Da ristrutturare".to_string()));
    columns0.push(("Caldaia autonoma".to_string(),"Appartamento".to_string()));
    let tablerow0:TableRow = TableRow { columns: columns0 };
    rows.push(tablerow0);

    let mut columns1:Vec<(String,String)> = Vec::new();
    columns1.push(("Via Cavour n.5".to_string(),"Roma".to_string()));
    columns1.push(("Mq. 150".to_string(),"Da ristrutturare".to_string()));
    columns1.push(("Caldaia autonoma".to_string(),"Appartamento".to_string()));
    let tablerow1:TableRow = TableRow { columns: columns1 };
    rows.push(tablerow1);

    let mut columns2:Vec<(String,String)> = Vec::new();
    columns2.push(("Via Cavour n.5".to_string(),"Roma".to_string()));
    columns2.push(("Mq. 150".to_string(),"Da ristrutturare".to_string()));
    columns2.push(("Caldaia autonoma".to_string(),"Appartamento".to_string()));
    let tablerow2:TableRow = TableRow { columns: columns2 };
    rows.push(tablerow2);


    let pagination:PaginationInfo = PaginationInfo{startPageItem:1, endPageItem:10, itemPages:10, totalItems:80};
    let tabledata:TableViewData = TableViewData { datatype:WinkEntity::Immobili, items: rows, pagination: pagination };

    cx.render(rsx!(
        div {
            Navbar(&cx, 1)
            FindBarImmobili(cx)
            TableView{data:tabledata}
        }
    ))
}