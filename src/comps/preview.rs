use dioxus::prelude::*;
use crate::comps::comment::Comment;
use crate::structs::StoryPageData;

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData)
}

#[component]
pub fn Preview() -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();

    match preview_state() {
        PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
        PreviewState::Loading => rsx! {"Loading..."},
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                    div { dangerous_inner_html: story.item.text }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}