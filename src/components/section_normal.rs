use leptos::*;
use leptos_markdown::Markdown;

/// Slides
#[component]
pub fn SectionNormal(#[prop(default = "")]title: &'static str, children: Children) -> impl IntoView {
    // children() returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { {child} })
        .collect::<Vec<_>>();

    view! {
        <section>
            <h2>{title}</h2>
            
            {children}
        </section>
    }

}