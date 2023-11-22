#![allow(non_snake_case, unused)]

use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};
use dioxus_router::prelude::{Link, Routable};
use super::super::commons::winkentityenum::WinkEntity;
use super::super::super::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct PaginationInfo{
    pub startPageItem: u32,
    pub endPageItem: u32, 
    pub itemPages: u8,
    pub totalItems: u32        
}
#[derive(Clone, Debug, PartialEq)]
pub struct TableRow{
    pub columns:Vec<(String,String)>
}
#[derive(Clone, Debug, PartialEq)]
pub struct TableViewData{
    pub datatype: WinkEntity,
    pub items: Vec<TableRow>,
    pub pagination: PaginationInfo
}

#[inline_props]
pub fn TableView(cx: Scope, data:TableViewData) -> Element {
    cx.render(rsx!(
        div {
            class:"shadow-xl rounded-lg border-2 bg-gray-100 mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",            
            div{                
                ul{
                    class:"divide-y divide-gray-100",
                    role:"list",
                    for item in data.items.iter() {
                        li{
                            class:"flex justify-between gap-x-6 py-5", 
                            div{                                
                                class:"flex min-w-0 gap-x-4",
                                Link{
                                    to: match data.datatype{
                                        WinkEntity::Immobili =>Route::Immobile {},
                                        WinkEntity::Anagrafiche =>Route::Anagrafica {},
                                        WinkEntity::Colloqui =>Route::Colloqui {},
                                        _ => Route::Mainwin {},
                                    },
                                    svg {
                                        class:"h-12 w-12 flex-none rounded-full bg-gray-50",
                                        fill:"currentColor",
                                        path {
                                            d:"M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z"
                                        },
                                    }                                    
                                }
                                for column in item.columns.iter(){
                                    div{
                                        class:"min-w-0 flex-auto",
                                        p{
                                            class:"text-sm font-semibold leading-6 text-gray-900",
                                            "{column.0}"
                                        }
                                        p{
                                            class:"mt-1 truncate text-xs leading-5 text-gray-500",
                                            "{column.1}"
                                        }
                                    }
                                }                    
                            }    
                        }
                    }
                }
            },
            div{
                class:"hidden sm:flex sm:flex-1 sm:items-center sm:justify-between",
                div{
                    p{
                        class:"text-sm text-gray-700",
                        "Elementi da ",
                        span{
                            class:"font-medium",
                            "{data.pagination.startPageItem}"
                        },
                        " a "
                        span{
                            class:"font-medium",
                            "{data.pagination.endPageItem}"
                        },
                        " di ",                  
                        span{
                            class:"font-medium",
                            "{data.pagination.totalItems}"
                        },
                    }
                },
                div{
                    class:"isolate inline-flex -space-x-px rounded-md shadow-sm",
                    "aria-label":"Pagination",
                    svg{
                        class:"h-10 w-5 cursor-pointer",
                        fill:"currentColor",
                        "viewBox":"0 0 20 20",
                        "aria-hidden":"true",
                        path{
                            "fill-rule":"evenodd",
                            "clip-rule":"evenodd",
                            d:"M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z" 
                        }
                    },
                    for c in 1..data.pagination.totalItems/data.pagination.itemPages as u32{
                        span{
                            class:"cursor-pointer relative z-10 inline-flex items-center bg-gray-100 px-4 py-2 text-sm font-semibold text-gray-900 focus:z-20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:bg-gray-600",
                            "{c}"
                        }    
                    }
                    svg{
                        class:"h-10 w-5 cursor-pointer",
                        fill:"currentColor",
                        "viewBox":"0 0 20 20",
                        "aria-hidden":"true",
                        path{
                            "fill-rule":"evenodd",
                            "clip-rule":"evenodd",
                            d:"M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z" 
                        }
                    },
                    

                }
            }
        }

    ))
}