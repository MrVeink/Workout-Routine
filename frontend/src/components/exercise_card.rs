use leptos::prelude::*;

#[component]
pub fn ExerciseCard(
    name: &'static str,
    exercise_id: &'static str,
    target: u32,
    current: u32,
    on_update: impl Fn(String, u32) + Clone + 'static,
) -> impl IntoView {
    let progress_percentage = if target > 0 {
        (current as f32 / target as f32 * 100.0).min(100.0)
    } else {
        0.0
    };

    let progress_class = if progress_percentage >= 100.0 {
        "progress-complete"
    } else if progress_percentage >= 50.0 {
        "progress-halfway"
    } else {
        "progress-start"
    };

    let on_decrement = {
        let on_update = on_update.clone();
        let exercise_id = exercise_id.to_string();
        move |_| {
            if current > 0 {
                on_update(exercise_id.clone(), current - 1);
            }
        }
    };

    let on_increment = {
        let on_update = on_update.clone();
        let exercise_id = exercise_id.to_string();
        move |_| {
            if current < target {
                on_update(exercise_id.clone(), current + 1);
            }
        }
    };

    view! {
        <div class="exercise" id=exercise_id>
            <h3>{name}</h3>
            <div class="counter">
                <button 
                    class="decrement"
                    disabled=move || current == 0
                    on:click=on_decrement
                >
                    "-"
                </button>
                <span class="count">{current}</span>
                <button 
                    class="increment"
                    disabled=move || current >= target
                    on:click=on_increment
                >
                    "+"
                </button>
            </div>
            <div class="target">
                "Target: " <span class="target-value">{target}</span>
            </div>
            <div class="progress-bar">
                <div 
                    class=format!("progress {}", progress_class)
                    style=format!("width: {}%", progress_percentage)
                >
                </div>
            </div>
        </div>
    }
}