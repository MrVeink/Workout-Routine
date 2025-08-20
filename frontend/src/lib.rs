use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

mod api;
mod components;
mod models;

use components::workout_tracker::WorkoutTracker;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Title text="Workout Tracker"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#4a90e2"/>
        
        <Router>
            <div class="app">
                <header>
                    <h1>"Workout Tracker"</h1>
                </header>
                
                <main>
                    <Routes>
                        <Route path="" view=WorkoutTracker/>
                    </Routes>
                </main>
                
                <footer>
                    <p>"Rust + Leptos + WebAssembly Workout Tracker"</p>
                </footer>
            </div>
        </Router>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    leptos::mount_to_body(|| view! { <App/> })
}