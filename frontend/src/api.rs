use gloo_net::http::Request;
use crate::models::{WorkoutDay, UserProgress, UpdateProgressRequest};

const API_BASE: &str = "http://localhost:3001/api";

pub async fn fetch_workouts() -> Result<Vec<WorkoutDay>, gloo_net::Error> {
    let response = Request::get(&format!("{}/workouts", API_BASE))
        .send()
        .await?;
    
    response.json::<Vec<WorkoutDay>>().await
}

pub async fn fetch_workout_by_date(date: &str) -> Result<WorkoutDay, gloo_net::Error> {
    let response = Request::get(&format!("{}/workouts/{}", API_BASE, date))
        .send()
        .await?;
    
    response.json::<WorkoutDay>().await
}

pub async fn fetch_progress(date: &str) -> Result<UserProgress, gloo_net::Error> {
    let response = Request::get(&format!("{}/progress/{}", API_BASE, date))
        .send()
        .await?;
    
    response.json::<UserProgress>().await
}

pub async fn update_progress(date: &str, exercise: &str, count: u32) -> Result<UserProgress, gloo_net::Error> {
    let request_body = UpdateProgressRequest {
        exercise: exercise.to_string(),
        count,
    };
    
    let response = Request::put(&format!("{}/progress/{}", API_BASE, date))
        .header("Content-Type", "application/json")
        .json(&request_body)?
        .send()
        .await?;
    
    response.json::<UserProgress>().await
}

pub async fn complete_workout(date: &str) -> Result<UserProgress, gloo_net::Error> {
    let response = Request::post(&format!("{}/progress/{}/complete", API_BASE, date))
        .send()
        .await?;
    
    response.json::<UserProgress>().await
}

pub async fn reset_progress(date: &str) -> Result<UserProgress, gloo_net::Error> {
    let response = Request::post(&format!("{}/progress/{}/reset", API_BASE, date))
        .send()
        .await?;
    
    response.json::<UserProgress>().await
}