use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::models::{WorkoutDay, UserProgress};
use crate::api;

#[component]
pub fn WorkoutTracker() -> impl IntoView {
    let (workouts, set_workouts) = signal(Vec::<WorkoutDay>::new());
    let (current_index, set_current_index) = signal(0);
    let (user_progress, set_user_progress) = signal(None::<UserProgress>);
    let (loading, set_loading) = signal(true);

    // Load workouts on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match api::fetch_workouts().await {
                Ok(workout_data) => {
                    set_workouts.set(workout_data);
                    set_loading.set(false);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load workouts: {:?}", e).into());
                    set_loading.set(false);
                }
            }
        });
    });

    // Load progress when current workout changes
    Effect::new(move |_| {
        let workouts_val = workouts.get();
        let index = current_index.get();
        
        if !workouts_val.is_empty() && index < workouts_val.len() {
            let current_workout = &workouts_val[index];
            let date = current_workout.date.clone();
            
            spawn_local(async move {
                match api::fetch_progress(&date).await {
                    Ok(progress) => set_user_progress.set(Some(progress)),
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to load progress: {:?}", e).into());
                    }
                }
            });
        }
    });

    let current_workout = move || {
        let workouts_val = workouts.get();
        let index = current_index.get();
        if workouts_val.is_empty() || index >= workouts_val.len() {
            return None;
        }
        Some(workouts_val[index].clone())
    };

    let on_prev = move |_: web_sys::Event| {
        let index = current_index.get();
        if index > 0 {
            set_current_index.set(index - 1);
        }
    };

    let on_next = move |_: web_sys::Event| {
        let index = current_index.get();
        let workouts_val = workouts.get();
        if index < workouts_val.len().saturating_sub(1) {
            set_current_index.set(index + 1);
        }
    };

    let update_exercise = move |exercise: String, count: u32| {
        if let Some(workout) = current_workout() {
            let date = workout.date.clone();
            spawn_local(async move {
                match api::update_progress(&date, &exercise, count).await {
                    Ok(progress) => set_user_progress.set(Some(progress)),
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to update progress: {:?}", e).into());
                    }
                }
            });
        }
    };

    let complete_workout = move |_: web_sys::Event| {
        if let Some(workout) = current_workout() {
            let date = workout.date.clone();
            spawn_local(async move {
                match api::complete_workout(&date).await {
                    Ok(progress) => {
                        set_user_progress.set(Some(progress));
                        // Show success message
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Workout completed! Great job! 💪")
                            .unwrap();
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to complete workout: {:?}", e).into());
                    }
                }
            });
        }
    };

    let reset_workout = move |_: web_sys::Event| {
        if let Some(workout) = current_workout() {
            let date = workout.date.clone();
            let confirmed = web_sys::window()
                .unwrap()
                .confirm_with_message("Are you sure you want to reset your progress for this day?")
                .unwrap_or(false);
            
            if confirmed {
                spawn_local(async move {
                    match api::reset_progress(&date).await {
                        Ok(progress) => set_user_progress.set(Some(progress)),
                        Err(e) => {
                            web_sys::console::error_1(&format!("Failed to reset progress: {:?}", e).into());
                        }
                    }
                });
            }
        }
    };

    view! {
        <div class="workout-container">
            {move || if loading.get() {
                view! { <div class="loading">"Loading workout data..."</div> }
            } else {
                let workouts_val = workouts.get();
                let index = current_index.get();
                
                if let Some(workout) = current_workout() {
                    view! {
                        <div class="date-navigation">
                            <button 
                                id="prev-date"
                                prop:disabled=move || current_index.get() == 0
                                on:click=on_prev
                            >
                                "◀"
                            </button>
                            <h2 id="current-date">{workout.date.clone()}</h2>
                            <button 
                                id="next-date"
                                prop:disabled=move || current_index.get() >= workouts.get().len().saturating_sub(1)
                                on:click=on_next
                            >
                                "▶"
                            </button>
                        </div>
                        
                        <div class="workout-card">
                            {if workout.is_rest_day() {
                                view! {
                                    <div class="rest-day-message">
                                        <h3>"Rest Day"</h3>
                                        <p>"Today is a scheduled rest day. Take time to recover and prepare for your next workout."</p>
                                    </div>
                                }
                            } else {
                                let exercise_handler = update_exercise.clone();
                                view! {
                                    <div class="exercise-list">
                                        <div class="exercise" id="sit_ups">
                                            <h3>"Sit-ups"</h3>
                                            <div class="counter">
                                                <button 
                                                    class="decrement"
                                                    prop:disabled=move || user_progress.get().map(|p| p.sit_ups).unwrap_or(0) == 0
                                                    on:click={
                                                        let handler = exercise_handler.clone();
                                                        move |_: web_sys::Event| {
                                                            let current = user_progress.get().map(|p| p.sit_ups).unwrap_or(0);
                                                            if current > 0 {
                                                                handler("sit_ups".to_string(), current - 1);
                                                            }
                                                        }
                                                    }
                                                >
                                                    "-"
                                                </button>
                                                <span class="count">{move || user_progress.get().map(|p| p.sit_ups).unwrap_or(0)}</span>
                                                <button 
                                                    class="increment"
                                                    prop:disabled=move || {
                                                        let current = user_progress.get().map(|p| p.sit_ups).unwrap_or(0);
                                                        current >= workout.sit_ups
                                                    }
                                                    on:click={
                                                        let handler = exercise_handler.clone();
                                                        move |_: web_sys::Event| {
                                                            let current = user_progress.get().map(|p| p.sit_ups).unwrap_or(0);
                                                            if current < workout.sit_ups {
                                                                handler("sit_ups".to_string(), current + 1);
                                                            }
                                                        }
                                                    }
                                                >
                                                    "+"
                                                </button>
                                            </div>
                                            <div class="target">
                                                "Target: " <span class="target-value">{workout.sit_ups}</span>
                                            </div>
                                            <div class="progress-bar">
                                                <div 
                                                    class="progress"
                                                    style:width=move || {
                                                        let current = user_progress.get().map(|p| p.sit_ups).unwrap_or(0);
                                                        let target = workout.sit_ups;
                                                        if target > 0 {
                                                            format!("{}%", (current as f32 / target as f32 * 100.0).min(100.0))
                                                        } else {
                                                            "0%".to_string()
                                                        }
                                                    }
                                                >
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="actions">
                                        <button 
                                            id="complete-workout"
                                            on:click=complete_workout
                                        >
                                            "Complete Workout"
                                        </button>
                                        <button 
                                            id="reset-workout"
                                            on:click=reset_workout
                                        >
                                            "Reset Progress"
                                        </button>
                                    </div>
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { <div class="error">"No workout data available"</div> }
                }
            }}
        </div>
    }
}