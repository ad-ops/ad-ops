mod components;

use dioxus::prelude::*;
use dioxus_router::{Router, Route};
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            components::navbar::navbar {}
            Route { to: "/", components::home::home {} }
            Route { to: "/clicky", components::clicky::clicky {} }
            Route { to: "", "Err 404 Route Not Found" }
        }
    ))
}
