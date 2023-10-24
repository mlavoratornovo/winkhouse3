#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;
use dioxus::{prelude::{*, GlobalAttributes}, html::{text, svg}};
use dioxus_router::prelude::{Link, Routable};
use super::mainwin::Mainwin;
use super::immobili::immobili::Immobili;
use super::anagrafiche::anagrafiche::Anagrafiche;
use super::colloqui::colloqui::Colloqui;
use super::super::Route;

pub fn Navbar(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            class:"mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
            div {
                class:"relative flex h-16 items-center justify-between",
                div {
                    class:"absolute inset-y-0 left-0 flex items-center sm:hidden",
                    button {
                        class:"relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white",
                        "aria-controls":"mobile-menu",
                        "aria-expanded":"false",
                        span {
                            class:"absolute-inset-0.5"
                        }
                        span {
                            class:"sr-only",
                            "Open main menu"
                        }
                        svg {
                            class:"block h-6 w-6",
                            fill:"none",
                            stroke:"currentColor",
                            "viewBox":"0 0 24 24",
                            "stroke-width":"1.5",
                            "aria-hidden":"true",
                            path {
                                d:"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                                "stroke-linecap":"round",
                                "stroke-linejoin":"round",
                            }
                        }
                        svg {
                            class:"block h-6 w-6",
                            fill:"none",
                            stroke:"currentColor",
                            "viewBox":"0 0 24 24",
                            "stroke-width":"1.5",
                            "aria-hidden":"true",
                            path {
                                d:"M6 18L18 6M6 6l12 12",
                                "stroke-linecap":"round",
                                "stroke-linejoin":"round",
                            }
                        }
                    }
                }
                div {
                    class:"flex flex-1 items-center justify-center sm:items-stretch sm:justify-start",
                    div {
                        class:"flex flex-shrink-0 items-center",
                        img {
                            class:"h-8 w-auto",
                            src:"logowinkhose.svg",
                            alt:"Winkhouse3"
                        }  
                    }
                    div {
                        class:"hidden sm:ml-6 sm:block",
                        div {
                            class:"flex space-x-4",
                            Link {
                                to:Route::Mainwin {},
                                class:"bg-gray-900 text-white rounded-md px-3 py-2 text-sm font-medium",                                                                
                                "Home"
                            }
                            Link {
                                to:Route::Immobili {},
                                class:"text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",                                       
                                "Immobii"
                            }                            
                            Link {
                                to:Route::Anagrafiche {},
                                class:"text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",                                      
                                "Anagrafiche"
                            }                            
                            Link {
                                to:Route::Colloqui {},
                                class:"text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",
                                "Colloqui"
                            }                             
                        }
                    }
                }
                div {
                    class:"absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                    button {
                        class:"relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800",
                        span {
                            class:"absolute-inset-1.5"
                        }
                        span {
                            class:"sr-only",
                            "View notifications"
                        }
                        svg {
                            class:"h-6 w-6",
                            fill:"none",
                            stroke:"currentColor",
                            "viewBox":"0 0 24 24",
                            "stroke-width":"1.5",
                            "aria-hidden":"true",
                            path {
                                d:"M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0",                                
                                "stroke-linejoin":"round",
                            }
                        }
                    }
                    div {
                        class:"relative ml-3",
                        div {
                            button {
                                class:"relative flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800",
                                id:"user-menu-button",
                                "aria-expanded":"false",
                                "aria-haspopup":"true",
                                span {
                                    class:"absolute-inset-1.5"
                                }
                                span {
                                    class:"sr-only",
                                    "Open user menu"
                                }
                                img {
                                    class:"h-8 w-8 rounded-full",
                                    src:""
                                }
                            }
                        }
                        div {
                            class:"absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                            role:"menu",
                            tabindex:"-1",
                            "aria-orientation":"vertical",
                            "aria-labelledby":"user-menu-button",
                            Link {
                                to:Route::Mainwin {},
                                class:"block px-4 py-2 text-sm text-gray-700",                                                                
                                "Home"
                            }
                            Link {
                                to:Route::Immobili {},
                                class:"block px-4 py-2 text-sm text-gray-700",                                       
                                "Immobii"
                            }                            
                            Link {
                                to:Route::Anagrafiche {},
                                class:"block px-4 py-2 text-sm text-gray-700",                                      
                                "Anagrafiche"
                            }                                    
                        }

                    }
                        
                }
            }
        }
        div {
            class:"sm:hidden",
            id:"mobile-menu",
            div {
                class:"space-y-1 px-2 pb-3 pt-2",
                Link {
                    to:Route::Mainwin {},
                    class:"bg-gray-900 text-white rounded-md px-3 py-2 text-sm font-medium",                                                                
                    "Home"
                }
                Link {
                    to:Route::Immobili {},
                    class:"text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",                                       
                    "Immobii"
                }                            
                Link {
                    to:Route::Anagrafiche {},
                    class:"text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",                                      
                    "Anagrafiche"
                }                            
                Link {
                    to:Route::Colloqui {},
                    class:"text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",
                    "Colloqui"
                }                             
            }
        }
    ))
}