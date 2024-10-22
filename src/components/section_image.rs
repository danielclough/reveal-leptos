// Import the necessary modules from Leptos and its markdown extension.
use leptos::*;
use leptos_markdown::Markdown;

/// A component for rendering a section with an image.
///
/// This component takes in several parameters to customize its behavior and appearance.
/// It can display a title, description, and transition effects when entering or exiting the view.
#[component]
pub fn SectionImage(
    /// Define a parameter `title` that defaults to an empty string if not provided.
    #[prop(default = "")] 
    /// The title of the section. Can be displayed using <h2>{title}</h2>.
    title: &'static str,
    
    /// Define a parameter `src` for the image source URL.
    src: &'static str,
    
    /// Define a parameter `alt` for the image alt text.
    alt: &'static str,
    
    /// Define a parameter `description` that defaults to an empty string if not provided.
    #[prop(default = "")] 
    /// A description of the section. Can be displayed as plain text.
    description: &'static str,
    
    /// Define a parameter `transition` that defaults to "fade-in slide-out" if not provided.
    #[prop(default = "fade-in slide-out")] 
    /// The transition effect to use when entering or exiting the view.
    transition: &'static str,
) -> impl IntoView {
    /// Render the section HTML element with a data-transition attribute set to the provided transition effect.
    view! {
        <section data-transition={move || transition}>
            // Use a Show component to render an h2 title only if it's not empty.
            <Show when= move || !title.is_empty()>
                <h2>{title}</h2>
            </Show>

            // Render the image HTML element with the provided source and alt text.
            <img style="margin-bottom: 0;width: 100%;max-height:80vh;" src=src alt=alt class="section-image"/>

            // Use a Show component to render an span only if it's not empty.
            <Show when= move || !description.is_empty()>
                <br/>
                <span>{description}</span>
            </Show>
        </section>
    }
}
