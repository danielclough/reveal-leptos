use leptos::*;
use leptos_markdown::Markdown;

/// Slides
#[component]
pub fn SectionTitle(title: &'static str) -> impl IntoView {
    view! {
        <section>
            <h2>{title}</h2>
        </section>
    }

}