use leptos::*;

use crate::components::section_image::SectionImage;

/// Read the Source
#[component]
pub fn ReadTheSource() -> impl IntoView {
    view! {
        <section>
            <SectionImage src="public/content/read-the-source-luke.jpeg" alt="Read the Source Luke"/>
            <SectionImage description="Not So Strange" src="public/content/index-html.png" alt="Index HTML"/>
            <SectionImage description="Yay! Javascript" src="public/content/index-init-reveal.png" alt="Index Init Reveal"/>
            <SectionImage description="Sections, like this." src="public/content/read-the-source-code.png" alt="Read The Source Component"/>
            <SectionImage description="Images, like this." src="public/content/section-image.png" alt="SectionImage Component"/>
            <SectionImage description="HTML & Markdown." src="public/content/section-about.png" alt="The About Component"/>
        </section>
    }
}
