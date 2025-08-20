use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutDay {
    pub date: String, // Format: "DD.MM.YYYY"
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

#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub exercise: String,
    pub count: u32,
}

type AppState = Arc<Mutex<HashMap<String, UserProgress>>>;

#[tokio::main]
async fn main() {
    // Initialize in-memory storage for user progress
    let progress_store: AppState = Arc::new(Mutex::new(HashMap::new()));

    // Build our application with routes
    let app = Router::new()
        .route("/api/workouts", get(list_workouts))
        .route("/api/workouts/:date", get(get_workout_by_date))
        .route("/api/progress/:date", get(get_progress))
        .route("/api/progress/:date", put(update_progress))
        .route("/api/progress/:date/complete", post(complete_workout))
        .route("/api/progress/:date/reset", post(reset_progress))
        .with_state(progress_store)
        .layer(CorsLayer::permissive());

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Backend server running on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}

// Handler to list all workout days
async fn list_workouts() -> Json<Vec<WorkoutDay>> {
    Json(get_workout_data())
}

// Handler to get workout by date
async fn get_workout_by_date(Path(date): Path<String>) -> Result<Json<WorkoutDay>, StatusCode> {
    let workouts = get_workout_data();
    match workouts.iter().find(|w| w.date == date) {
        Some(workout) => Ok(Json(workout.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Handler to get user progress for a specific date
async fn get_progress(
    Path(date): Path<String>,
    State(state): State<AppState>,
) -> Json<UserProgress> {
    let progress_store = state.lock().unwrap();
    let progress = progress_store
        .get(&date)
        .cloned()
        .unwrap_or(UserProgress {
            date: date.clone(),
            sit_ups: 0,
            push_ups: 0,
            squats: 0,
            pull_ups: 0,
        });
    Json(progress)
}

// Handler to update user progress
async fn update_progress(
    Path(date): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<UpdateProgressRequest>,
) -> Result<Json<UserProgress>, StatusCode> {
    let mut progress_store = state.lock().unwrap();
    
    let mut progress = progress_store
        .get(&date)
        .cloned()
        .unwrap_or(UserProgress {
            date: date.clone(),
            sit_ups: 0,
            push_ups: 0,
            squats: 0,
            pull_ups: 0,
        });

    match request.exercise.as_str() {
        "sit_ups" => progress.sit_ups = request.count,
        "push_ups" => progress.push_ups = request.count,
        "squats" => progress.squats = request.count,
        "pull_ups" => progress.pull_ups = request.count,
        _ => return Err(StatusCode::BAD_REQUEST),
    }

    progress_store.insert(date, progress.clone());
    Ok(Json(progress))
}

// Handler to complete workout (set all exercises to target values)
async fn complete_workout(
    Path(date): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserProgress>, StatusCode> {
    let workouts = get_workout_data();
    let workout = workouts.iter().find(|w| w.date == date);
    
    match workout {
        Some(w) => {
            let progress = UserProgress {
                date: date.clone(),
                sit_ups: w.sit_ups,
                push_ups: w.push_ups,
                squats: w.squats,
                pull_ups: w.pull_ups,
            };
            
            let mut progress_store = state.lock().unwrap();
            progress_store.insert(date, progress.clone());
            Ok(Json(progress))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Handler to reset progress for a date
async fn reset_progress(
    Path(date): Path<String>,
    State(state): State<AppState>,
) -> Json<UserProgress> {
    let progress = UserProgress {
        date: date.clone(),
        sit_ups: 0,
        push_ups: 0,
        squats: 0,
        pull_ups: 0,
    };
    
    let mut progress_store = state.lock().unwrap();
    progress_store.insert(date, progress.clone());
    Json(progress)
}

// Static workout data (converted from original workout-data.js)
fn get_workout_data() -> Vec<WorkoutDay> {
    vec![
        WorkoutDay { date: "18.08.2025".to_string(), sit_ups: 18, push_ups: 7, squats: 15, pull_ups: 13 },
        WorkoutDay { date: "19.08.2025".to_string(), sit_ups: 19, push_ups: 8, squats: 15, pull_ups: 14 },
        WorkoutDay { date: "20.08.2025".to_string(), sit_ups: 21, push_ups: 8, squats: 16, pull_ups: 15 },
        WorkoutDay { date: "21.08.2025".to_string(), sit_ups: 0, push_ups: 0, squats: 0, pull_ups: 0 }, // Rest day
        WorkoutDay { date: "22.08.2025".to_string(), sit_ups: 22, push_ups: 9, squats: 16, pull_ups: 16 },
        WorkoutDay { date: "23.08.2025".to_string(), sit_ups: 24, push_ups: 10, squats: 17, pull_ups: 17 },
        WorkoutDay { date: "24.08.2025".to_string(), sit_ups: 25, push_ups: 10, squats: 17, pull_ups: 17 },
        WorkoutDay { date: "25.08.2025".to_string(), sit_ups: 0, push_ups: 0, squats: 0, pull_ups: 0 }, // Rest day
        WorkoutDay { date: "26.08.2025".to_string(), sit_ups: 26, push_ups: 11, squats: 18, pull_ups: 18 },
        WorkoutDay { date: "27.08.2025".to_string(), sit_ups: 27, push_ups: 12, squats: 18, pull_ups: 19 },
        WorkoutDay { date: "28.08.2025".to_string(), sit_ups: 29, push_ups: 13, squats: 18, pull_ups: 20 },
        WorkoutDay { date: "29.08.2025".to_string(), sit_ups: 0, push_ups: 0, squats: 0, pull_ups: 0 }, // Rest day
        WorkoutDay { date: "30.08.2025".to_string(), sit_ups: 29, push_ups: 14, squats: 19, pull_ups: 21 },
        WorkoutDay { date: "31.08.2025".to_string(), sit_ups: 30, push_ups: 14, squats: 19, pull_ups: 21 },
        WorkoutDay { date: "01.09.2025".to_string(), sit_ups: 31, push_ups: 15, squats: 20, pull_ups: 22 },
        WorkoutDay { date: "02.09.2025".to_string(), sit_ups: 0, push_ups: 0, squats: 0, pull_ups: 0 }, // Rest day
    ]
}