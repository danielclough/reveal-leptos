use leptos::*;
use leptos_markdown::Markdown;

/// Slides
#[component]
pub fn Slides() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);
    view! {
        <section>
            <Markdown src="# Reactive Power!"/>
            { move || counter() }
            <button on:click=move |_| set_counter.update(|v| *v += 1)> 
                "Increment Now!"
            </button>
        </section>
        <section>
            <section>
                <Markdown src="# Markdown Power!"/>
            </section>
            <section>
                <Markdown src="# Markdown Power!"/>
            </section>
        </section>
    }
}
