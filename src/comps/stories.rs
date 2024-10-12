use dioxus::prelude::*;
use crate::comps::story_list::StoryListing;
use crate::structs::StoryItem;
use crate::services::get_stories;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_stories(10));

    match &*stories.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for story in list {
                    StoryListing { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => rsx! {"An error occurred while fetching stories {err}"},
        None => rsx! {"Loading items"},
    }
}
