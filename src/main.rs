#![allow(non_snake_case, unused)]
use dioxus::prelude::*;//, GlobalAttributes};
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use components::mainwin::*;
use components::immobili::immobili::*;
use components::anagrafiche::anagrafiche::*;
use components::colloqui::colloqui::*;
use components::categorie::categorie::*;
mod components;
fn main() {
    // LaunchBuilder::new(app).launch();
    
    let config:LaunchBuilder::<FullstackRouterConfig<Route>> = LaunchBuilder::<FullstackRouterConfig<Route>>::router();    
    #[cfg(feature = "ssr")]
    config
        .incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )
        .launch();

    #[cfg(not(feature = "ssr"))]
    config.launch();
}
#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[route("/")]
    App {},

    #[route("/home")]
	Mainwin {},
	
	#[route("/immobili")]
	Immobili {},

	#[route("/anagrafiche")]
	Anagrafiche {},

    #[route("/colloqui")]
	Colloqui {},

	#[route("/categorie")]
	Categorie {},

}

fn App(cx: Scope) -> Element {    

    cx.render(rsx! {
        components::mainwin::Mainwin(&cx)
    })
}

