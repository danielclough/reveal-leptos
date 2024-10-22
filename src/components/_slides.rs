use crate::components::content::about::About;
use crate::components::content::read_the_source::ReadTheSource;
use leptos::*;
// use leptos_markdown::Markdown;

/// About Me
#[component]
pub fn Slides() -> impl IntoView {
    view! {
        // About Me
        <About />

        // Leptos and Reveal.js Integration
        // <LeptosRevealJsIntegration/>

        // Read the Source Code!
        <ReadTheSource/>

        // FreeSlides
        // <FreeSlides/>
    }
}
