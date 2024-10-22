use leptos::*;
use leptos_markdown::Markdown;

/// Slides
#[component]
pub fn SectionMarkdown(src: &'static str) -> impl IntoView {
    view! {
        <section>
            <Markdown src />
        </section>
    }
}