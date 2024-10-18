use crate::components::_slides::Slides;
use leptos::*;

// use wasm_bindgen::prelude::*;
// #[wasm_bindgen(start)]
// pub fn start() {
//     let window = window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");

//     let closure = Closure::wrap(Box::new(move || {
//         initialize_reveal().expect("Failed to initialize Reveal.js");
//     }) as Box<dyn FnMut()>);

//     document.add_event_listener_with_callback("DOMContentLoaded", closure.as_ref().unchecked_ref())
//         .expect("Failed to add event listener");

//     closure.forget(); // Prevent the closure from being dropped
// }

// #[wasm_bindgen]
// pub fn initialize_reveal() -> Result<(), JsValue> {
//     let window = window().expect("no global `window` exists");
//     let reveal = window.get("Reveal").expect("Reveal.js not loaded");

//     let config = Object::new();
//     let dependencies = js_sys::Array::new();

//     // Add dependencies
//     add_dependency(&dependencies, "public/reveal/plugin/markdown/marked.js", None, None)?;
//     add_dependency(&dependencies, "public/reveal/plugin/markdown/markdown.js", None, None)?;
//     add_dependency(&dependencies, "public/reveal/plugin/notes/notes.js", Some(true), None)?;
//     add_dependency(&dependencies, "public/reveal/plugin/highlight/highlight.js", Some(true), Some("hljs.initHighlightingOnLoad()"))?;

//     js_sys::Reflect::set(&config, &"dependencies".into(), &dependencies)?;

//     // Initialize Reveal
//     let init_method = js_sys::Reflect::get(&reveal, &"initialize".into())?;
//     let init_func = init_method.dyn_ref::<js_sys::Function>().unwrap();
//     init_func.call1(&reveal, &config)?;

//     Ok(())
// }

// fn add_dependency(
//     dependencies: &js_sys::Array,
//     src: &str,
//     async_: Option<bool>,
//     callback: Option<&str>,
// ) -> Result<(), JsValue> {
//     let dep = Object::new();
//     js_sys::Reflect::set(&dep, &"src".into(), &JsValue::from_str(src))?;

//     if let Some(async_val) = async_ {
//         js_sys::Reflect::set(&dep, &"async".into(), &JsValue::from_bool(async_val))?;
//     }

//     if let Some(callback_str) = callback {
//         let callback_fn = js_sys::Function::new_no_args(callback_str);
//         js_sys::Reflect::set(&dep, &"callback".into(), &callback_fn)?;
//     }

//     dependencies.push(&dep);
//     Ok(())
// }

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
        <div class="reveal">
	        <div class="slides">
                <Slides />
            </div>
        </div>

        </ErrorBoundary>
    }
}
