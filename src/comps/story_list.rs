use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::comps::PreviewState;
use crate::services::hackernews::resolve_story;
use crate::StoryItem;
use crate::structs::StoryPageData;

#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        id,
        ..
    } = story();
    let full_story = use_signal(|| None);

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if score == 1 { " point" } else { " points" });
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            onmouseenter: move |_event| { resolve_story(full_story, preview_state, id) },
            div { font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| { resolve_story(full_story, preview_state, id) },
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div { display: "flex", flex_direction: "row", color: "gray",
                div { "{score}" }
                div { padding_left: "0.5rem", "by {by}" }
                div { padding_left: "0.5rem", "{time}" }
                div { padding_left: "0.5rem", "{comments}" }
            }
        }
    }
}