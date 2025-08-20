# Workout Routine Tracker - Rust Edition

A modern workout tracking application built with Rust, featuring a **Leptos** frontend compiled to WebAssembly and an **Axum** backend REST API.

## Overview

This is a complete rewrite of the original JavaScript workout tracker in Rust, maintaining all core functionality while leveraging the performance and safety benefits of Rust and WebAssembly.

### Features

- 📅 **Date-based workout navigation** - Navigate through your workout schedule
- 💪 **Exercise tracking** - Track sit-ups, push-ups, squats, and pull-ups
- 📊 **Progress visualization** - Visual progress bars with color-coded feedback
- ✅ **Complete workout** - Mark entire workout as complete with one click
- 🔄 **Reset progress** - Reset daily progress when needed
- 🛌 **Rest day support** - Automatic rest day detection and display
- 💾 **Progress persistence** - Backend API stores your workout progress
- 🚀 **Fast WebAssembly frontend** - Compiled Rust code running in the browser

## Architecture

```
├── backend/          # Axum REST API server
├── frontend/         # Leptos WebAssembly application
└── Cargo.toml        # Rust workspace configuration
```

### Backend (Axum)

The backend provides a REST API for workout data and progress tracking:

- **GET `/api/workouts`** - List all workout days
- **GET `/api/workouts/{date}`** - Get specific workout by date
- **GET `/api/progress/{date}`** - Get user progress for a date
- **PUT `/api/progress/{date}`** - Update exercise progress
- **POST `/api/progress/{date}/complete`** - Complete entire workout
- **POST `/api/progress/{date}/reset`** - Reset progress for a date

### Frontend (Leptos + WebAssembly)

The frontend is a single-page application built with Leptos that compiles to WebAssembly:

- **Reactive UI** - Built with Leptos reactive primitives
- **Component-based** - Modular components for exercises, navigation, etc.
- **Type-safe API** - Shared models between frontend and backend
- **Modern CSS** - Responsive design with CSS variables

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for WebAssembly compilation

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd Workout-Routine
   ```

2. **Install wasm-pack** (if not already installed)
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

### Running the Application

#### Backend Server

1. **Start the backend API server:**
   ```bash
   cd backend
   cargo run
   ```
   
   The server will start on `http://localhost:3001`

#### Frontend Application

1. **Build the frontend** (in a new terminal):
   ```bash
   cd frontend
   ./build.sh
   ```

2. **Serve the frontend** (requires Python or any static file server):
   ```bash
   # Using Python (recommended)
   cd frontend/static
   python -m http.server 8000
   
   # Or using Node.js
   npx serve .
   
   # Or any other static file server
   ```

3. **Open your browser** and navigate to `http://localhost:8000`

### Development Workflow

#### Backend Development

```bash
cd backend
cargo run          # Run the server
cargo check        # Check for compilation errors
cargo test         # Run tests (when added)
```

#### Frontend Development

```bash
cd frontend
./build.sh         # Rebuild WebAssembly
# Then refresh your browser
```

For faster development, you can use `wasm-pack` directly:
```bash
wasm-pack build --target web --out-dir static/pkg --dev
```

## API Documentation

### Workout Data Model

```rust
{
    "id": "uuid",
    "date": "18.08.2025",
    "sit_ups": 18,
    "push_ups": 7,
    "squats": 15,
    "pull_ups": 13
}
```

### Progress Data Model

```rust
{
    "date": "18.08.2025",
    "sit_ups": 10,
    "push_ups": 5,
    "squats": 8,
    "pull_ups": 3
}
```

### Example API Calls

```bash
# Get all workouts
curl http://localhost:3001/api/workouts

# Get specific workout
curl http://localhost:3001/api/workouts/18.08.2025

# Update progress
curl -X PUT http://localhost:3001/api/progress/18.08.2025 \
  -H "Content-Type: application/json" \
  -d '{"exercise": "sit_ups", "count": 15}'

# Complete workout
curl -X POST http://localhost:3001/api/progress/18.08.2025/complete

# Reset progress
curl -X POST http://localhost:3001/api/progress/18.08.2025/reset
```

## Project Structure

```
Workout-Routine/
├── Cargo.toml                    # Workspace configuration
├── backend/
│   ├── Cargo.toml               # Backend dependencies
│   └── src/
│       └── main.rs              # Axum server with API routes
├── frontend/
│   ├── Cargo.toml               # Frontend dependencies
│   ├── .cargo/config.toml       # WebAssembly build config
│   ├── build.sh                 # Build script
│   ├── static/
│   │   ├── index.html           # Main HTML file
│   │   └── style.css            # Styling
│   └── src/
│       ├── main.rs              # Leptos app entry point
│       ├── api.rs               # API client functions
│       ├── models.rs            # Shared data models
│       └── components/
│           ├── mod.rs
│           ├── workout_tracker.rs    # Main app component
│           ├── exercise_card.rs      # Exercise UI component
│           └── date_navigation.rs    # Date navigation component
└── README.md                    # This file
```

## Technologies Used

### Backend
- **[Axum](https://github.com/tokio-rs/axum)** - Modern async web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Serde](https://serde.rs/)** - Serialization framework
- **[Tower](https://github.com/tower-rs/tower)** - Middleware and utilities

### Frontend
- **[Leptos](https://leptos.dev/)** - Reactive web framework
- **[WebAssembly](https://webassembly.org/)** - Compilation target
- **[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)** - Rust-WebAssembly bindings
- **[gloo](https://github.com/rustwasm/gloo)** - Web API bindings

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test both backend and frontend
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Migration from JavaScript

This Rust version maintains feature parity with the original JavaScript implementation:

- ✅ All original functionality preserved
- ✅ Same UI/UX experience
- ✅ Compatible data format
- ✅ Progressive enhancement with better performance
- ✅ Type safety throughout the application
- ✅ Modern development workflow

The main improvements over the original:
- **Type Safety**: Rust's type system prevents runtime errors
- **Performance**: WebAssembly compilation for faster execution
- **Maintainability**: Better code organization and error handling
- **Scalability**: Proper backend API for future enhancements