use leptos::prelude::*;

#[component]
pub fn DateNavigation(
    current_index: usize,
    total_workouts: usize,
    current_date: String,
    on_change: impl Fn(usize) + 'static,
) -> impl IntoView {
    let prev_disabled = current_index == 0;
    let next_disabled = current_index >= total_workouts.saturating_sub(1);
    
    let on_prev = move |_| {
        if current_index > 0 {
            on_change(current_index - 1);
        }
    };
    
    let on_next = move |_| {
        if current_index < total_workouts.saturating_sub(1) {
            on_change(current_index + 1);
        }
    };

    view! {
        <div class="date-navigation">
            <button 
                id="prev-date"
                disabled=prev_disabled
                on:click=on_prev
            >
                "◀"
            </button>
            <h2 id="current-date">{current_date}</h2>
            <button 
                id="next-date"
                disabled=next_disabled
                on:click=on_next
            >
                "▶"
            </button>
        </div>
    }
}