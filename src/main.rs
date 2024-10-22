use leptos::*;
use reveal_leptos::App;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    browser_panic_hook::set_once(|| {
        browser_panic_hook::CustomBody(Box::new(|details| {
            // render new body
            format!("…")
        }))
    });

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
