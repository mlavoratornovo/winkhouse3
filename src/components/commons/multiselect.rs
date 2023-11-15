#![allow(non_snake_case, unused)]

use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}};

#[derive(Clone, Debug, PartialEq)]
pub struct MultiSelectDataItem{
    pub key:i32,
    pub value_return:String,
    pub value_display:String
}
#[derive(Clone, Debug, PartialEq)]
pub struct MultiSelectData{
    pub key: i32,
    pub label_name:String,
    pub items: Vec<MultiSelectDataItem>
}

#[inline_props]
pub fn MultiSelect(cx: Scope, data: MultiSelectData) -> Element {
    let mut statoComponente = use_state(cx, || "hidden");
    // let mut posizioneMenuX = use_state(cx, || 0.0);
    // let mut posizioneMenuY = use_state(cx, || 0.0);
    cx.render(rsx!(
        div{
            class:"static border-t border-gray-200 px-4 py-6",
            h3 {
                class:"-mx-2 -my-3 flow-root",
                
                // Expand/collapse section button -->
                span {
                    class:"ml-6 flex items-center text-sm",
                    "{data.label_name}"
                //   <!-- Expand icon, show/hide based on section open state. -->
                    svg {
                        class:"h-5 w-5 cursor-pointer",                        
                        fill:"currentColor",
                        onmouseup: move |event| {
                            statoComponente.set("z-10");
                            // posizioneMenuX.set(event.screen_coordinates().x);
                            // posizioneMenuY.set(event.screen_coordinates().y);
                        },
                        "viewBox":"0 0 20 20",
                        "aria-hidden":"true",
                        path {
                            d:"M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z"
                        }
                    }
                    //    <!-- Collapse icon, show/hide based on section open state. -->
                    svg {
                        class:"h-5 w-5 cursor-pointer", 
                        fill:"currentColor",
                        onmouseup: move |event| {
                            statoComponente.set("hidden");
                        },
                        "viewBox":"0 0 20 20",
                        "aria-hidden":"true",
                        path {                                    
                            d:"M4 10a.75.75 0 01.75-.75h10.5a.75.75 0 010 1.5H4.75A.75.75 0 014 10z",
                            "fill-rule":"evenodd",
                            "clip-rule":"evenodd"
                        }
                    }
                }
            }
            div {
                class:"h-1 flex items-center text-sm",
            }
            // <!-- Filter section, show/hide based on section state. -->
            div {
                class:"{statoComponente} rounded-lg border-1 absolute bg-gray-100 border border-gray-500 p-1 m-2",
                id:"filter-section-mobile-{data.key}",
                div {
                    class:"space-y-1",
                    for item in data.items.iter() {
                        div {
                            class:"flex items-center",
                            input {
                                id:"filter-mobile-color-0",
                                name:"{item.key}",
                                value:"{item.value_return}",
                                r#type:"checkbox",
                                class:"h-4 w-4 rounded border-gray-300 text-gray-600 focus:ring-gray-500"
                            }
                            label {
                                class:"ml-3 min-w-0 flex-1 text-gray-500 text-xs",
                                r#for:"filter-mobile-color-0",
                                "{item.value_display}"
                            }
                        }
                    }
                }
            }
        }
    
    ))
}