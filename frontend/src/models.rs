use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutDay {
    pub date: String,
    pub sit_ups: u32,
    pub push_ups: u32,
    pub squats: u32,
    pub pull_ups: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub date: String,
    pub sit_ups: u32,
    pub push_ups: u32,
    pub squats: u32,
    pub pull_ups: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProgressRequest {
    pub exercise: String,
    pub count: u32,
}

impl WorkoutDay {
    pub fn is_rest_day(&self) -> bool {
        self.sit_ups == 0 && self.push_ups == 0 && self.squats == 0 && self.pull_ups == 0
    }
}

impl UserProgress {
    pub fn get_exercise_count(&self, exercise: &str) -> u32 {
        match exercise {
            "sit_ups" => self.sit_ups,
            "push_ups" => self.push_ups,
            "squats" => self.squats,
            "pull_ups" => self.pull_ups,
            _ => 0,
        }
    }

    pub fn set_exercise_count(&mut self, exercise: &str, count: u32) {
        match exercise {
            "sit_ups" => self.sit_ups = count,
            "push_ups" => self.push_ups = count,
            "squats" => self.squats = count,
            "pull_ups" => self.pull_ups = count,
            _ => {}
        }
    }
}