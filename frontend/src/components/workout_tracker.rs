use leptos::prelude::*;
use crate::models::{WorkoutDay, UserProgress};
use crate::api;
use crate::components::{date_navigation::DateNavigation, exercise_card::ExerciseCard};

#[component]
pub fn WorkoutTracker() -> impl IntoView {
    let (workouts, set_workouts) = create_signal(Vec::<WorkoutDay>::new());
    let (current_index, set_current_index) = create_signal(0);
    let (user_progress, set_user_progress) = create_signal(None::<UserProgress>);
    let (loading, set_loading) = create_signal(true);

    // Load workouts on mount
    create_effect(move |_| {
        spawn_local(async move {
            match api::fetch_workouts().await {
                Ok(workout_data) => {
                    set_workouts(workout_data);
                    set_loading(false);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load workouts: {:?}", e).into());
                    set_loading(false);
                }
            }
        });
    });

    // Load progress when current workout changes
    create_effect(move |_| {
        let workouts_val = workouts();
        let index = current_index();
        
        if !workouts_val.is_empty() && index < workouts_val.len() {
            let current_workout = &workouts_val[index];
            let date = current_workout.date.clone();
            
            spawn_local(async move {
                match api::fetch_progress(&date).await {
                    Ok(progress) => set_user_progress(Some(progress)),
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to load progress: {:?}", e).into());
                    }
                }
            });
        }
    });

    let current_workout = move || {
        let workouts_val = workouts();
        let index = current_index();
        if workouts_val.is_empty() || index >= workouts_val.len() {
            return None;
        }
        Some(workouts_val[index].clone())
    };

    let on_date_change = move |new_index: usize| {
        set_current_index(new_index);
    };

    let update_exercise = move |exercise: String, count: u32| {
        if let Some(workout) = current_workout() {
            let date = workout.date.clone();
            spawn_local(async move {
                match api::update_progress(&date, &exercise, count).await {
                    Ok(progress) => set_user_progress(Some(progress)),
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to update progress: {:?}", e).into());
                    }
                }
            });
        }
    };

    let complete_workout = move |_| {
        if let Some(workout) = current_workout() {
            let date = workout.date.clone();
            spawn_local(async move {
                match api::complete_workout(&date).await {
                    Ok(progress) => {
                        set_user_progress(Some(progress));
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

    let reset_workout = move |_| {
        if let Some(workout) = current_workout() {
            let date = workout.date.clone();
            let confirmed = web_sys::window()
                .unwrap()
                .confirm_with_message("Are you sure you want to reset your progress for this day?")
                .unwrap_or(false);
            
            if confirmed {
                spawn_local(async move {
                    match api::reset_progress(&date).await {
                        Ok(progress) => set_user_progress(Some(progress)),
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
            {move || if loading() {
                view! { <div class="loading">"Loading workout data..."</div> }.into_view()
            } else {
                let workouts_val = workouts();
                let index = current_index();
                
                if let Some(workout) = current_workout() {
                    view! {
                        <DateNavigation
                            current_index=index
                            total_workouts=workouts_val.len()
                            current_date=workout.date.clone()
                            on_change=on_date_change
                        />
                        
                        <div class="workout-card">
                            {if workout.is_rest_day() {
                                view! {
                                    <div class="rest-day-message">
                                        <h3>"Rest Day"</h3>
                                        <p>"Today is a scheduled rest day. Take time to recover and prepare for your next workout."</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="exercise-list">
                                        <ExerciseCard
                                            name="Sit-ups"
                                            exercise_id="sit_ups"
                                            target=workout.sit_ups
                                            current=user_progress().map(|p| p.sit_ups).unwrap_or(0)
                                            on_update=update_exercise.clone()
                                        />
                                        <ExerciseCard
                                            name="Push-ups"
                                            exercise_id="push_ups"
                                            target=workout.push_ups
                                            current=user_progress().map(|p| p.push_ups).unwrap_or(0)
                                            on_update=update_exercise.clone()
                                        />
                                        <ExerciseCard
                                            name="Squats"
                                            exercise_id="squats"
                                            target=workout.squats
                                            current=user_progress().map(|p| p.squats).unwrap_or(0)
                                            on_update=update_exercise.clone()
                                        />
                                        <ExerciseCard
                                            name="Pull-ups"
                                            exercise_id="pull_ups"
                                            target=workout.pull_ups
                                            current=user_progress().map(|p| p.pull_ups).unwrap_or(0)
                                            on_update=update_exercise.clone()
                                        />
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
                                }.into_view()
                            }}
                        </div>
                    }.into_view()
                } else {
                    view! { <div class="error">"No workout data available"</div> }.into_view()
                }
            }}
        </div>
    }
}