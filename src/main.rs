#![allow(non_snake_case)]

mod comps;
mod structs;
mod services;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use serde::{Deserialize, Serialize};

use structs::*;
use comps::Stories;
use crate::comps::{Preview, PreviewState};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    launch(App);
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { width: "50%", Stories { } }
            div { width: "50%", Preview { } }
        }
    }
}
