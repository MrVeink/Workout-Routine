use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::console;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
//
// For more details see
// https://github.com/rustwasm/console_error_panic_hook#readme
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_panic_hook;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkoutDay {
    pub date: String,
    pub sit_ups: u32,
    pub push_ups: u32,
    pub squats: u32,
    pub pull_ups: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Progress {
    pub sit_ups: u32,
    pub push_ups: u32,
    pub squats: u32,
    pub pull_ups: u32,
}

#[wasm_bindgen]
pub struct WorkoutTracker {
    workout_data: Vec<WorkoutDay>,
    user_progress: HashMap<String, Progress>,
    current_date_index: usize,
}

#[wasm_bindgen]
impl WorkoutTracker {
    #[wasm_bindgen(constructor)]
    pub fn new(workout_data_json: &str) -> Result<WorkoutTracker, JsValue> {
        // Parse the workout data from JSON
        let workout_data: Vec<WorkoutDay> = serde_json::from_str(workout_data_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse workout data: {}", e)))?;
        
        let user_progress = HashMap::new();
        
        Ok(WorkoutTracker {
            workout_data,
            user_progress,
            current_date_index: 0,
        })
    }

    #[wasm_bindgen]
    pub fn load_progress(&mut self, progress_json: &str) -> Result<(), JsValue> {
        if !progress_json.is_empty() {
            self.user_progress = serde_json::from_str(progress_json)
                .map_err(|e| JsValue::from_str(&format!("Failed to parse progress: {}", e)))?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_progress_json(&self) -> String {
        serde_json::to_string(&self.user_progress).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn get_current_workout(&self) -> JsValue {
        if let Some(workout) = self.workout_data.get(self.current_date_index) {
            serde_wasm_bindgen::to_value(workout).unwrap_or(JsValue::NULL)
        } else {
            JsValue::NULL
        }
    }

    #[wasm_bindgen]
    pub fn get_current_progress(&self) -> JsValue {
        let current_date = &self.workout_data[self.current_date_index].date;
        if let Some(progress) = self.user_progress.get(current_date) {
            serde_wasm_bindgen::to_value(progress).unwrap_or(JsValue::NULL)
        } else {
            serde_wasm_bindgen::to_value(&Progress {
                sit_ups: 0,
                push_ups: 0,
                squats: 0,
                pull_ups: 0,
            }).unwrap_or(JsValue::NULL)
        }
    }

    #[wasm_bindgen]
    pub fn increment_exercise(&mut self, exercise: &str) -> bool {
        let current_date = self.workout_data[self.current_date_index].date.clone();
        let current_workout = &self.workout_data[self.current_date_index];
        
        let progress = self.user_progress.entry(current_date).or_insert(Progress {
            sit_ups: 0,
            push_ups: 0,
            squats: 0,
            pull_ups: 0,
        });

        match exercise {
            "sit-ups" => {
                if progress.sit_ups < current_workout.sit_ups {
                    progress.sit_ups += 1;
                    true
                } else {
                    false
                }
            },
            "push-ups" => {
                if progress.push_ups < current_workout.push_ups {
                    progress.push_ups += 1;
                    true
                } else {
                    false
                }
            },
            "squats" => {
                if progress.squats < current_workout.squats {
                    progress.squats += 1;
                    true
                } else {
                    false
                }
            },
            "pull-ups" => {
                if progress.pull_ups < current_workout.pull_ups {
                    progress.pull_ups += 1;
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    #[wasm_bindgen]
    pub fn decrement_exercise(&mut self, exercise: &str) -> bool {
        let current_date = self.workout_data[self.current_date_index].date.clone();
        
        let progress = self.user_progress.entry(current_date).or_insert(Progress {
            sit_ups: 0,
            push_ups: 0,
            squats: 0,
            pull_ups: 0,
        });

        match exercise {
            "sit-ups" => {
                if progress.sit_ups > 0 {
                    progress.sit_ups -= 1;
                    true
                } else {
                    false
                }
            },
            "push-ups" => {
                if progress.push_ups > 0 {
                    progress.push_ups -= 1;
                    true
                } else {
                    false
                }
            },
            "squats" => {
                if progress.squats > 0 {
                    progress.squats -= 1;
                    true
                } else {
                    false
                }
            },
            "pull-ups" => {
                if progress.pull_ups > 0 {
                    progress.pull_ups -= 1;
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    #[wasm_bindgen]
    pub fn complete_workout(&mut self) {
        let current_date = self.workout_data[self.current_date_index].date.clone();
        let current_workout = &self.workout_data[self.current_date_index];
        
        self.user_progress.insert(current_date, Progress {
            sit_ups: current_workout.sit_ups,
            push_ups: current_workout.push_ups,
            squats: current_workout.squats,
            pull_ups: current_workout.pull_ups,
        });
    }

    #[wasm_bindgen]
    pub fn reset_workout(&mut self) {
        let current_date = self.workout_data[self.current_date_index].date.clone();
        
        self.user_progress.insert(current_date, Progress {
            sit_ups: 0,
            push_ups: 0,
            squats: 0,
            pull_ups: 0,
        });
    }

    #[wasm_bindgen]
    pub fn can_go_previous(&self) -> bool {
        self.current_date_index > 0
    }

    #[wasm_bindgen]
    pub fn can_go_next(&self) -> bool {
        self.current_date_index < self.workout_data.len() - 1
    }

    #[wasm_bindgen]
    pub fn go_previous(&mut self) -> bool {
        if self.can_go_previous() {
            self.current_date_index -= 1;
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn go_next(&mut self) -> bool {
        if self.can_go_next() {
            self.current_date_index += 1;
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn go_to_date_index(&mut self, index: usize) -> bool {
        if index < self.workout_data.len() {
            self.current_date_index = index;
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn get_current_date_index(&self) -> usize {
        self.current_date_index
    }

    #[wasm_bindgen]
    pub fn get_workout_data_length(&self) -> usize {
        self.workout_data.len()
    }

    #[wasm_bindgen]
    pub fn is_rest_day(&self) -> bool {
        let workout = &self.workout_data[self.current_date_index];
        workout.sit_ups == 0 && workout.push_ups == 0 && workout.squats == 0 && workout.pull_ups == 0
    }

    #[wasm_bindgen]
    pub fn get_workout_at_index(&self, index: usize) -> JsValue {
        if let Some(workout) = self.workout_data.get(index) {
            serde_wasm_bindgen::to_value(workout).unwrap_or(JsValue::NULL)
        } else {
            JsValue::NULL
        }
    }

    #[wasm_bindgen]
    pub fn is_workout_completed(&self, date_index: usize) -> bool {
        if let Some(workout) = self.workout_data.get(date_index) {
            if let Some(progress) = self.user_progress.get(&workout.date) {
                progress.sit_ups >= workout.sit_ups &&
                progress.push_ups >= workout.push_ups &&
                progress.squats >= workout.squats &&
                progress.pull_ups >= workout.pull_ups
            } else {
                false
            }
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn format_date(&self, date_str: &str) -> String {
        // Parse date string in format "DD.MM.YYYY"
        let parts: Vec<&str> = date_str.split('.').collect();
        if parts.len() != 3 {
            return date_str.to_string();
        }

        let day = parts[0].parse::<u32>().unwrap_or(1);
        let month = parts[1].parse::<u32>().unwrap_or(1);
        let year = parts[2].parse::<i32>().unwrap_or(2025);

        // Create date and format it
        let month_names = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        
        let weekday_names = [
            "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
        ];

        // Simple day of week calculation (this is approximate)
        let month_idx = if month > 12 { 0 } else { (month - 1) as usize };
        let month_name = month_names.get(month_idx).unwrap_or(&"January");
        
        // For simplicity, we'll use a basic weekday rotation
        let weekday_idx = ((day + month + year as u32) % 7) as usize;
        let weekday = weekday_names.get(weekday_idx).unwrap_or(&"Monday");

        format!("{}, {} {}, {}", weekday, month_name, day, year)
    }
}

// Utility functions that can be called from JavaScript

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_panic_hook();
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log!("Hello, {}!", name);
}
