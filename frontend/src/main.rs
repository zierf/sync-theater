#![recursion_limit = "128"]

mod app;

use app::{App, Props};

#[cfg(feature = "wee_alloc")]
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

fn main() {
    #[cfg(feature = "stack_trace")]
    // enable feature "std" to show rust stack trace instead of cryptic "RuntimeError: unreachable executed"
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let title = "Sync-Theater.rs".to_owned();
    let content_id = "main-content".to_owned();

    // mount components in content container
    let main_content = gloo::utils::document()
        .get_element_by_id(&content_id)
        .expect("can't find main content");

    yew::start_app_with_props_in_element::<App>(main_content, Props { name: title });
}
