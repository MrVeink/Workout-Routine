// This file contains the workout data converted from your CSV

const workoutData = [
    {date: "18.08.2025", sitUps: 18, pushUps: 7, squats: 15, pullUps: 13},
    {date: "19.08.2025", sitUps: 19, pushUps: 8, squats: 15, pullUps: 14},
    {date: "20.08.2025", sitUps: 21, pushUps: 8, squats: 16, pullUps: 15},
    {date: "21.08.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "22.08.2025", sitUps: 22, pushUps: 9, squats: 16, pullUps: 16},
    {date: "23.08.2025", sitUps: 24, pushUps: 10, squats: 17, pullUps: 17},
    {date: "24.08.2025", sitUps: 25, pushUps: 10, squats: 17, pullUps: 17},
    {date: "25.08.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "26.08.2025", sitUps: 26, pushUps: 11, squats: 18, pullUps: 18},
    {date: "27.08.2025", sitUps: 27, pushUps: 12, squats: 18, pullUps: 19},
    {date: "28.08.2025", sitUps: 29, pushUps: 13, squats: 18, pullUps: 20},
    {date: "29.08.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "30.08.2025", sitUps: 29, pushUps: 14, squats: 19, pullUps: 21},
    {date: "31.08.2025", sitUps: 30, pushUps: 14, squats: 19, pullUps: 21},
    {date: "01.09.2025", sitUps: 31, pushUps: 15, squats: 20, pullUps: 22},
    {date: "02.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "03.09.2025", sitUps: 31, pushUps: 16, squats: 21, pullUps: 23},
    {date: "04.09.2025", sitUps: 31, pushUps: 16, squats: 21, pullUps: 23},
    {date: "05.09.2025", sitUps: 32, pushUps: 17, squats: 21, pullUps: 23},
    {date: "06.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "07.09.2025", sitUps: 33, pushUps: 18, squats: 22, pullUps: 24},
    {date: "08.09.2025", sitUps: 34, pushUps: 19, squats: 22, pullUps: 25},
    {date: "09.09.2025", sitUps: 35, pushUps: 19, squats: 23, pullUps: 26},
    {date: "10.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "11.09.2025", sitUps: 35, pushUps: 20, squats: 23, pullUps: 26},
    {date: "12.09.2025", sitUps: 36, pushUps: 21, squats: 24, pullUps: 27},
    {date: "13.09.2025", sitUps: 37, pushUps: 21, squats: 24, pullUps: 27},
    {date: "14.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "15.09.2025", sitUps: 38, pushUps: 22, squats: 25, pullUps: 28},
    {date: "16.09.2025", sitUps: 39, pushUps: 23, squats: 25, pullUps: 29},
    {date: "17.09.2025", sitUps: 40, pushUps: 24, squats: 26, pullUps: 30},
    {date: "18.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "19.09.2025", sitUps: 41, pushUps: 25, squats: 27, pullUps: 31},
    {date: "20.09.2025", sitUps: 41, pushUps: 26, squats: 28, pullUps: 32},
    {date: "21.09.2025", sitUps: 42, pushUps: 27, squats: 29, pullUps: 33},
    {date: "22.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "23.09.2025", sitUps: 42, pushUps: 28, squats: 30, pullUps: 33},
    {date: "24.09.2025", sitUps: 42, pushUps: 29, squats: 31, pullUps: 34},
    {date: "25.09.2025", sitUps: 43, pushUps: 30, squats: 32, pullUps: 35},
    {date: "26.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "27.09.2025", sitUps: 44, pushUps: 31, squats: 33, pullUps: 36},
    {date: "28.09.2025", sitUps: 45, pushUps: 32, squats: 34, pullUps: 37},
    {date: "29.09.2025", sitUps: 47, pushUps: 33, squats: 35, pullUps: 38},
    {date: "30.09.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "01.10.2025", sitUps: 47, pushUps: 35, squats: 36, pullUps: 39},
    {date: "02.10.2025", sitUps: 48, pushUps: 35, squats: 37, pullUps: 40},
    {date: "03.10.2025", sitUps: 49, pushUps: 36, squats: 38, pullUps: 41},
    {date: "04.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "05.10.2025", sitUps: 50, pushUps: 38, squats: 39, pullUps: 42},
    {date: "06.10.2025", sitUps: 51, pushUps: 38, squats: 40, pullUps: 43},
    {date: "07.10.2025", sitUps: 52, pushUps: 39, squats: 41, pullUps: 44},
    {date: "08.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "09.10.2025", sitUps: 52, pushUps: 41, squats: 42, pullUps: 45},
    {date: "10.10.2025", sitUps: 53, pushUps: 41, squats: 43, pullUps: 46},
    {date: "11.10.2025", sitUps: 54, pushUps: 42, squats: 44, pullUps: 47},
    {date: "12.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "13.10.2025", sitUps: 55, pushUps: 44, squats: 45, pullUps: 48},
    {date: "14.10.2025", sitUps: 56, pushUps: 45, squats: 46, pullUps: 49},
    {date: "15.10.2025", sitUps: 56, pushUps: 45, squats: 47, pullUps: 49},
    {date: "16.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "17.10.2025", sitUps: 57, pushUps: 47, squats: 48, pullUps: 51},
    {date: "18.10.2025", sitUps: 58, pushUps: 48, squats: 49, pullUps: 52},
    {date: "19.10.2025", sitUps: 59, pushUps: 48, squats: 50, pullUps: 52},
    {date: "20.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "21.10.2025", sitUps: 60, pushUps: 50, squats: 51, pullUps: 54},
    {date: "22.10.2025", sitUps: 61, pushUps: 51, squats: 52, pullUps: 55},
    {date: "23.10.2025", sitUps: 61, pushUps: 51, squats: 53, pullUps: 55},
    {date: "24.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "25.10.2025", sitUps: 62, pushUps: 53, squats: 54, pullUps: 56},
    {date: "26.10.2025", sitUps: 63, pushUps: 54, squats: 55, pullUps: 57},
    {date: "27.10.2025", sitUps: 64, pushUps: 55, squats: 56, pullUps: 58},
    {date: "28.10.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "29.10.2025", sitUps: 65, pushUps: 56, squats: 57, pullUps: 59},
    {date: "30.10.2025", sitUps: 65, pushUps: 57, squats: 58, pullUps: 60},
    {date: "31.10.2025", sitUps: 66, pushUps: 58, squats: 59, pullUps: 61},
    {date: "01.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "02.11.2025", sitUps: 67, pushUps: 59, squats: 60, pullUps: 62},
    {date: "03.11.2025", sitUps: 68, pushUps: 60, squats: 61, pullUps: 63},
    {date: "04.11.2025", sitUps: 69, pushUps: 61, squats: 62, pullUps: 64},
    {date: "05.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "06.11.2025", sitUps: 69, pushUps: 62, squats: 63, pullUps: 65},
    {date: "07.11.2025", sitUps: 70, pushUps: 63, squats: 64, pullUps: 66},
    {date: "08.11.2025", sitUps: 71, pushUps: 64, squats: 65, pullUps: 67},
    {date: "09.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "10.11.2025", sitUps: 72, pushUps: 65, squats: 66, pullUps: 68},
    {date: "11.11.2025", sitUps: 73, pushUps: 66, squats: 67, pullUps: 69},
    {date: "12.11.2025", sitUps: 74, pushUps: 67, squats: 68, pullUps: 70},
    {date: "13.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "14.11.2025", sitUps: 74, pushUps: 68, squats: 69, pullUps: 70},
    {date: "15.11.2025", sitUps: 75, pushUps: 69, squats: 70, pullUps: 71},
    {date: "16.11.2025", sitUps: 76, pushUps: 70, squats: 71, pullUps: 72},
    {date: "17.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "18.11.2025", sitUps: 77, pushUps: 72, squats: 72, pullUps: 74},
    {date: "19.11.2025", sitUps: 78, pushUps: 72, squats: 73, pullUps: 74},
    {date: "20.11.2025", sitUps: 78, pushUps: 73, squats: 74, pullUps: 75},
    {date: "21.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "22.11.2025", sitUps: 79, pushUps: 75, squats: 75, pullUps: 76},
    {date: "23.11.2025", sitUps: 80, pushUps: 75, squats: 76, pullUps: 77},
    {date: "24.11.2025", sitUps: 81, pushUps: 76, squats: 77, pullUps: 78},
    {date: "25.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "26.11.2025", sitUps: 82, pushUps: 78, squats: 78, pullUps: 79},
    {date: "27.11.2025", sitUps: 83, pushUps: 78, squats: 79, pullUps: 80},
    {date: "28.11.2025", sitUps: 83, pushUps: 79, squats: 80, pullUps: 81},
    {date: "29.11.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "30.11.2025", sitUps: 84, pushUps: 81, squats: 81, pullUps: 82},
    {date: "01.12.2025", sitUps: 85, pushUps: 82, squats: 82, pullUps: 83},
    {date: "02.12.2025", sitUps: 86, pushUps: 82, squats: 83, pullUps: 84},
    {date: "03.12.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "04.12.2025", sitUps: 87, pushUps: 84, squats: 84, pullUps: 85},
    {date: "05.12.2025", sitUps: 87, pushUps: 85, squats: 85, pullUps: 86},
    {date: "06.12.2025", sitUps: 88, pushUps: 85, squats: 86, pullUps: 86},
    {date: "07.12.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "08.12.2025", sitUps: 89, pushUps: 87, squats: 87, pullUps: 88},
    {date: "09.12.2025", sitUps: 90, pushUps: 88, squats: 88, pullUps: 89},
    {date: "10.12.2025", sitUps: 91, pushUps: 88, squats: 89, pullUps: 89},
    {date: "11.12.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "12.12.2025", sitUps: 92, pushUps: 90, squats: 90, pullUps: 91},
    {date: "13.12.2025", sitUps: 92, pushUps: 91, squats: 91, pullUps: 91},
    {date: "14.12.2025", sitUps: 93, pushUps: 92, squats: 92, pullUps: 92},
    {date: "15.12.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "16.12.2025", sitUps: 94, pushUps: 93, squats: 93, pullUps: 93},
    {date: "17.12.2025", sitUps: 95, pushUps: 94, squats: 94, pullUps: 94},
    {date: "18.12.2025", sitUps: 96, pushUps: 95, squats: 95, pullUps: 95},
    {date: "19.12.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "20.12.2025", sitUps: 96, pushUps: 96, squats: 96, pullUps: 96},
    {date: "21.12.2025", sitUps: 97, pushUps: 97, squats: 97, pullUps: 97},
    {date: "22.12.2025", sitUps: 98, pushUps: 98, squats: 98, pullUps: 98},
    {date: "23.12.2025", sitUps: 0, pushUps: 0, squats: 0, pullUps: 0}, // Rest day
    {date: "24.12.2025", sitUps: 99, pushUps: 99, squats: 99, pullUps: 99},
    {date: "25.12.2025", sitUps: 100, pushUps: 100, squats: 100, pullUps: 100}
];