#![recursion_limit="128"]

mod app;

use app::{
    App,
    Props,
};

fn main() {
    let title = "Sync-Theater.rs".to_owned();
    let content_id = "main-content".to_owned();

    // mount components in content container
    let main_content = gloo::utils::document()
        .get_element_by_id(&content_id)
        .expect("can't find main content");

    yew::start_app_with_props_in_element::<App>(main_content, Props {
        name: title
    });
}
