use leptos::*;
use leptos_markdown::Markdown;
// use leptos_markdown::Markdown;
use leptos_meta::Stylesheet;

use crate::components::section_markdown::SectionMarkdown;

/// About
#[component]
pub fn About() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);
    let emojis = ["😉","😍","🤩","😵","😵‍💫","😎","🥹","🤖","💖","❤️‍🔥","💯","💫","💪","🧙","🧙‍♂️","🧙‍♀️","💃","🕺","⭐","🌟","🌠","🪄","🎉","🧨","🎇","🔥",];

    view! {
        <section>
            <section>
              <Markdown src="#
## Reactive Slideshows!

Built with Rust/Wasm (Leptos, Trunk) and JavaScript (Reveal.JS)!
#"/>
                <h1>{ move || emojis[counter() % 26] }</h1>
                <button on:click=move |_| set_counter.update(|v| *v += 1)> 
                    "Thank you, may I have another!?"
                </button>
            </section>
            
            <section>
                <Stylesheet href="public/css/about-me.css"/>
                <div class="container">
                    <div class="image-container">
                        <img src="public/content/daniel-clough.jpg" alt="Daniel Clough" class="profile-picture" />
                        <h1>Daniel Clough</h1>
                    </div>
                    <div class="info-container">
                        <blockquote>Software Developer, Photographer, Tea Master</blockquote>
                        <img style="margin-bottom:none;" src="public/content/danielc-us-qr.gif" alt="QR Code" class="qrcode" />
                        <br />
                        <a href="https://danielc.us">danielc.us</a>
                    </div>
                </div>
            </section>

            <section>
                <h1>
                    Leptos
                </h1>

                <iframe src="https://leptos.dev/" width="100%" height="600px"></iframe>
            </section>

            <section>
                <h1>
                    Reveal.JS
                </h1>

                <iframe src="https://revealjs.com/" width="100%" height="600px"></iframe>
            </section>
        </section>
    }
}

// <SectionMarkdown src="#
//# Title
//#" />
