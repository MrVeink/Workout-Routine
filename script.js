import init, { WorkoutTracker, init_panic_hook } from './pkg/workout_tracker_wasm.js';
import { workoutDataRust } from './workout-data.js';

let tracker = null;

async function initApp() {
    try {
        // Initialize the wasm module
        await init();
        
        // Set up panic hook for better error messages
        init_panic_hook();
        
        // Create the workout tracker with the workout data
        const workoutDataJson = JSON.stringify(workoutDataRust);
        tracker = new WorkoutTracker(workoutDataJson);
        
        // Load saved progress from localStorage
        const savedProgress = localStorage.getItem('workoutProgressRust');
        if (savedProgress) {
            tracker.load_progress(savedProgress);
        }
        
        console.log('WebAssembly module initialized successfully');
        
        // Initialize the UI
        initializeUI();
        updateDisplay();
        
    } catch (error) {
        console.error('Failed to initialize WebAssembly module:', error);
        // Fall back to original JavaScript implementation
        await loadFallbackScript();
    }
}

function initializeUI() {
    // DOM elements
    const currentDateElement = document.getElementById('current-date');
    const prevDateButton = document.getElementById('prev-date');
    const nextDateButton = document.getElementById('next-date');
    const completeWorkoutButton = document.getElementById('complete-workout');
    const resetWorkoutButton = document.getElementById('reset-workout');
    const showCalendarButton = document.getElementById('show-calendar');
    const hideCalendarButton = document.getElementById('hide-calendar');
    const calendarView = document.querySelector('.calendar-view');
    const calendarContainer = document.getElementById('calendar-container');
    
    // Event listeners
    prevDateButton.addEventListener('click', () => {
        if (tracker.go_previous()) {
            updateDisplay();
        }
    });
    
    nextDateButton.addEventListener('click', () => {
        if (tracker.go_next()) {
            updateDisplay();
        }
    });
    
    // Add event listeners to all exercise counters
    document.querySelectorAll('.exercise').forEach(exercise => {
        const exerciseId = exercise.id;
        const decrementButton = exercise.querySelector('.decrement');
        const incrementButton = exercise.querySelector('.increment');
        
        decrementButton.addEventListener('click', () => {
            if (tracker.decrement_exercise(exerciseId)) {
                saveProgress();
                updateDisplay();
            }
        });
        
        incrementButton.addEventListener('click', () => {
            if (tracker.increment_exercise(exerciseId)) {
                saveProgress();
                updateDisplay();
            }
        });
    });
    
    completeWorkoutButton.addEventListener('click', () => {
        tracker.complete_workout();
        saveProgress();
        updateDisplay();
        alert('Workout completed! Great job! 💪');
    });
    
    resetWorkoutButton.addEventListener('click', () => {
        if (confirm('Are you sure you want to reset your progress for this day?')) {
            tracker.reset_workout();
            saveProgress();
            updateDisplay();
        }
    });
    
    showCalendarButton.addEventListener('click', () => {
        document.querySelector('.workout-card').style.display = 'none';
        document.querySelector('.date-navigation').style.display = 'none';
        document.querySelector('.calendar-toggle').style.display = 'none';
        calendarView.style.display = 'block';
        renderCalendar();
    });
    
    hideCalendarButton.addEventListener('click', () => {
        document.querySelector('.workout-card').style.display = 'block';
        document.querySelector('.date-navigation').style.display = 'flex';
        document.querySelector('.calendar-toggle').style.display = 'block';
        calendarView.style.display = 'none';
    });
}

function updateDisplay() {
    const currentWorkout = tracker.get_current_workout();
    const currentProgress = tracker.get_current_progress();
    
    // Update date display
    const currentDateElement = document.getElementById('current-date');
    currentDateElement.textContent = tracker.format_date(currentWorkout.date);
    
    // Check if it's a rest day
    const isRestDay = tracker.is_rest_day();
    const exerciseContainer = document.querySelector('.exercise-list');
    
    if (isRestDay) {
        // Show rest day message
        if (!document.querySelector('.rest-day-message')) {
            const restDayMessage = document.createElement('div');
            restDayMessage.className = 'rest-day-message';
            restDayMessage.innerHTML = `
                <h3>Rest Day</h3>
                <p>Today is a scheduled rest day. Take time to recover and prepare for your next workout.</p>
            `;
            exerciseContainer.innerHTML = '';
            exerciseContainer.appendChild(restDayMessage);
        }
    } else {
        // Regular workout day - restore exercise elements if needed
        if (document.querySelector('.rest-day-message')) {
            location.reload(); // Simple way to restore the original HTML structure
        }
        
        // Update each exercise
        updateExercise('sit-ups', 'sit_ups', currentWorkout, currentProgress);
        updateExercise('push-ups', 'push_ups', currentWorkout, currentProgress);
        updateExercise('squats', 'squats', currentWorkout, currentProgress);
        updateExercise('pull-ups', 'pull_ups', currentWorkout, currentProgress);
    }
    
    // Update navigation buttons
    const prevDateButton = document.getElementById('prev-date');
    const nextDateButton = document.getElementById('next-date');
    prevDateButton.disabled = !tracker.can_go_previous();
    nextDateButton.disabled = !tracker.can_go_next();
}

function updateExercise(exerciseId, propertyName, currentWorkout, currentProgress) {
    const exerciseElement = document.getElementById(exerciseId);
    const targetElement = exerciseElement.querySelector('.target-value');
    const countElement = exerciseElement.querySelector('.count');
    const progressElement = exerciseElement.querySelector('.progress');
    
    const targetCount = currentWorkout[propertyName];
    const userCount = currentProgress[propertyName];
    
    targetElement.textContent = targetCount;
    countElement.textContent = userCount;
    
    // Update progress bar
    const progressPercentage = targetCount > 0 ? (userCount / targetCount) * 100 : 0;
    progressElement.style.width = `${progressPercentage}%`;
    
    // Change color based on progress
    if (progressPercentage >= 100) {
        progressElement.style.backgroundColor = 'var(--success-color)';
    } else if (progressPercentage >= 50) {
        progressElement.style.backgroundColor = 'var(--warning-color)';
    } else {
        progressElement.style.backgroundColor = 'var(--primary-color)';
    }
}

function renderCalendar() {
    const calendarContainer = document.getElementById('calendar-container');
    calendarContainer.innerHTML = '';
    
    // Add day labels
    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    days.forEach(day => {
        const dayLabel = document.createElement('div');
        dayLabel.className = 'calendar-day-label';
        dayLabel.textContent = day;
        dayLabel.style.textAlign = 'center';
        dayLabel.style.fontWeight = 'bold';
        dayLabel.style.padding = '4px';
        calendarContainer.appendChild(dayLabel);
    });
    
    // Get the first date from workout data
    const firstWorkout = tracker.get_workout_at_index(0);
    const firstDateParts = firstWorkout.date.split('.');
    const firstDate = new Date(firstDateParts[2], firstDateParts[1] - 1, firstDateParts[0]);
    
    // Add offset for proper day alignment
    const firstDayOfWeek = firstDate.getDay();
    for (let i = 0; i < firstDayOfWeek; i++) {
        const emptyDay = document.createElement('div');
        calendarContainer.appendChild(emptyDay);
    }
    
    // Add calendar days
    const dataLength = tracker.get_workout_data_length();
    for (let index = 0; index < dataLength; index++) {
        const workout = tracker.get_workout_at_index(index);
        const dateParts = workout.date.split('.');
        const date = new Date(dateParts[2], dateParts[1] - 1, dateParts[0]);
        
        const dayElement = document.createElement('div');
        dayElement.className = 'calendar-day';
        
        // Check if it's today
        const today = new Date();
        const isToday = date.getDate() === today.getDate() && 
                      date.getMonth() === today.getMonth() && 
                      date.getFullYear() === today.getFullYear();
        
        if (isToday) {
            dayElement.classList.add('today');
        }
        
        // Check if it's a rest day
        const isRestDay = workout.sit_ups === 0 && 
                        workout.push_ups === 0 && 
                        workout.squats === 0 && 
                        workout.pull_ups === 0;
        
        if (isRestDay) {
            dayElement.classList.add('rest-day');
            dayElement.innerHTML = `<div>${dateParts[0]}</div><small>REST</small>`;
        } else {
            // Check if completed
            const isCompleted = tracker.is_workout_completed(index);
            
            if (isCompleted) {
                dayElement.classList.add('completed');
            }
            
            dayElement.innerHTML = `<div>${dateParts[0]}</div>`;
        }
        
        // Navigate to this day when clicked
        dayElement.addEventListener('click', () => {
            tracker.go_to_date_index(index);
            updateDisplay();
            document.getElementById('hide-calendar').click();
        });
        
        calendarContainer.appendChild(dayElement);
    }
}

function saveProgress() {
    const progressJson = tracker.get_progress_json();
    localStorage.setItem('workoutProgressRust', progressJson);
}

async function loadFallbackScript() {
    // Load the original JavaScript implementation as fallback
    console.log('Loading fallback JavaScript implementation...');
    
    // Remove the WASM script and load the original
    const script = document.createElement('script');
    script.src = 'script.js';
    script.onload = () => {
        console.log('Fallback script loaded successfully');
    };
    script.onerror = () => {
        console.error('Failed to load fallback script');
    };
    document.head.appendChild(script);
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', initApp);