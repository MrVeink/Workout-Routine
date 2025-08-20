document.addEventListener('DOMContentLoaded', function() {
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
    const exerciseContainer = document.querySelector('.exercise-list');
    
    // App state
    let currentDateIndex = 0;
    let userProgress = loadProgress();
    
    // Initialize
    updateDisplay();
    
    // Event listeners
    prevDateButton.addEventListener('click', () => {
        if (currentDateIndex > 0) {
            currentDateIndex--;
            updateDisplay();
        }
    });
    
    nextDateButton.addEventListener('click', () => {
        if (currentDateIndex < workoutData.length - 1) {
            currentDateIndex++;
            updateDisplay();
        }
    });
    
    // Add event listeners to all exercise counters
    document.querySelectorAll('.exercise').forEach(exercise => {
        const exerciseId = exercise.id;
        const decrementButton = exercise.querySelector('.decrement');
        const incrementButton = exercise.querySelector('.increment');
        const countElement = exercise.querySelector('.count');
        
        decrementButton.addEventListener('click', () => {
            const currentDate = workoutData[currentDateIndex].date;
            const currentCount = parseInt(countElement.textContent);
            
            if (currentCount > 0) {
                userProgress[currentDate] = userProgress[currentDate] || {
                    sitUps: 0, pushUps: 0, squats: 0, pullUps: 0
                };
                userProgress[currentDate][exerciseIdToProperty(exerciseId)] = currentCount - 1;
                saveProgress();
                updateDisplay();
            }
        });
        
        incrementButton.addEventListener('click', () => {
            const currentDate = workoutData[currentDateIndex].date;
            const currentCount = parseInt(countElement.textContent);
            const targetCount = getTargetCount(exerciseId, currentDateIndex);
            
            userProgress[currentDate] = userProgress[currentDate] || {
                sitUps: 0, pushUps: 0, squats: 0, pullUps: 0
            };
            userProgress[currentDate][exerciseIdToProperty(exerciseId)] = Math.min(currentCount + 1, targetCount);
            saveProgress();
            updateDisplay();
        });
    });
    
    completeWorkoutButton.addEventListener('click', () => {
        const currentDate = workoutData[currentDateIndex].date;
        
        userProgress[currentDate] = {
            sitUps: workoutData[currentDateIndex].sitUps,
            pushUps: workoutData[currentDateIndex].pushUps,
            squats: workoutData[currentDateIndex].squats,
            pullUps: workoutData[currentDateIndex].pullUps
        };
        
        saveProgress();
        updateDisplay();
        alert('Workout completed! Great job! 💪');
    });
    
    resetWorkoutButton.addEventListener('click', () => {
        if (confirm('Are you sure you want to reset your progress for this day?')) {
            const currentDate = workoutData[currentDateIndex].date;
            
            userProgress[currentDate] = {
                sitUps: 0, pushUps: 0, squats: 0, pullUps: 0
            };
            
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
    
    // Functions
    function updateDisplay() {
        const currentWorkout = workoutData[currentDateIndex];
        currentDateElement.textContent = formatDate(currentWorkout.date);
        
        // Check if it's a rest day
        const isRestDay = currentWorkout.sitUps === 0 && 
                         currentWorkout.pushUps === 0 && 
                         currentWorkout.squats === 0 && 
                         currentWorkout.pullUps === 0;
        
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
            updateExercise('sit-ups', 'sitUps', currentWorkout);
            updateExercise('push-ups', 'pushUps', currentWorkout);
            updateExercise('squats', 'squats', currentWorkout);
            updateExercise('pull-ups', 'pullUps', currentWorkout);
        }
        
        // Disable/enable navigation buttons
        prevDateButton.disabled = currentDateIndex === 0;
        nextDateButton.disabled = currentDateIndex === workoutData.length - 1;
    }
    
    function updateExercise(exerciseId, propertyName, currentWorkout) {
        const exerciseElement = document.getElementById(exerciseId);
        const targetElement = exerciseElement.querySelector('.target-value');
        const countElement = exerciseElement.querySelector('.count');
        const progressElement = exerciseElement.querySelector('.progress');
        
        const targetCount = currentWorkout[propertyName];
        const currentDate = currentWorkout.date;
        const userCount = userProgress[currentDate] && userProgress[currentDate][propertyName] 
            ? userProgress[currentDate][propertyName] 
            : 0;
        
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
        const firstDateParts = workoutData[0].date.split('.');
        const firstDate = new Date(firstDateParts[2], firstDateParts[1] - 1, firstDateParts[0]);
        
        // Add offset for proper day alignment
        const firstDayOfWeek = firstDate.getDay();
        for (let i = 0; i < firstDayOfWeek; i++) {
            const emptyDay = document.createElement('div');
            calendarContainer.appendChild(emptyDay);
        }
        
        // Add calendar days
        workoutData.forEach((workout, index) => {
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
            const isRestDay = workout.sitUps === 0 && 
                            workout.pushUps === 0 && 
                            workout.squats === 0 && 
                            workout.pullUps === 0;
            
            if (isRestDay) {
                dayElement.classList.add('rest-day');
                dayElement.innerHTML = `<div>${dateParts[0]}</div><small>REST</small>`;
            } else {
                // Check if completed
                const isCompleted = userProgress[workout.date] && 
                                  userProgress[workout.date].sitUps >= workout.sitUps &&
                                  userProgress[workout.date].pushUps >= workout.pushUps &&
                                  userProgress[workout.date].squats >= workout.squats &&
                                  userProgress[workout.date].pullUps >= workout.pullUps;
                
                if (isCompleted) {
                    dayElement.classList.add('completed');
                }
                
                dayElement.innerHTML = `<div>${dateParts[0]}</div>`;
            }
            
            // Navigate to this day when clicked
            dayElement.addEventListener('click', () => {
                currentDateIndex = index;
                updateDisplay();
                hideCalendarButton.click();
            });
            
            calendarContainer.appendChild(dayElement);
        });
    }
    
    function exerciseIdToProperty(id) {
        switch(id) {
            case 'sit-ups': return 'sitUps';
            case 'push-ups': return 'pushUps';
            case 'squats': return 'squats';
            case 'pull-ups': return 'pullUps';
            default: return '';
        }
    }
    
    function getTargetCount(exerciseId, dateIndex) {
        const propertyName = exerciseIdToProperty(exerciseId);
        return workoutData[dateIndex][propertyName];
    }
    
    function formatDate(dateString) {
        const parts = dateString.split('.');
        const date = new Date(parts[2], parts[1] - 1, parts[0]);
        
        const options = { 
            weekday: 'long', 
            year: 'numeric', 
            month: 'long', 
            day: 'numeric' 
        };
        
        return date.toLocaleDateString(undefined, options);
    }
    
    function saveProgress() {
        localStorage.setItem('workoutProgress', JSON.stringify(userProgress));
    }
    
    function loadProgress() {
        const savedProgress = localStorage.getItem('workoutProgress');
        return savedProgress ? JSON.parse(savedProgress) : {};
    }
});