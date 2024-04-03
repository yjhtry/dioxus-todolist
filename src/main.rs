#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/todo")]
    Todo {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Todo() -> Element {
    rsx! {
        "Todo list"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            "Home page"
        }
    }
}
