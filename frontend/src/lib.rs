use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::*;

mod api;
mod components;
mod models;

use components::workout_tracker::WorkoutTracker;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div class="app">
            <header>
                <h1>"Workout Tracker"</h1>
            </header>
            
            <main>
                <WorkoutTracker/>
            </main>
            
            <footer>
                <p>"Rust + Leptos + WebAssembly Workout Tracker"</p>
            </footer>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| view! { <App/> })
}