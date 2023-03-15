use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    exercise_name: String,
    sets: i32,
    repetitions: i32,
    load: i32,
}

impl Exercise {
    pub fn total_workload(&self) -> i32 {
        self.sets * self.repetitions * self.load
    }
}

impl std::fmt::Display for Exercise {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}x{} with {} kgs.",
            self.exercise_name, self.sets, self.repetitions, self.load
        )
    }
}

impl FromStr for Exercise {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut exercise_data = s.split(':').take(4);

        let exercise_name = String::from(exercise_data.next().unwrap());

        let [sets, repetitions, load] = <[i32; 3]>::try_from(
            exercise_data
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>(),
        )
        .unwrap();

        Ok(Exercise {
            exercise_name,
            sets,
            repetitions,
            load,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    workout_date: String,
    workout_type: String,
    targeted_muscles: String,
    exercises: Vec<Exercise>,
}

impl std::fmt::Display for Workout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut msg_string = format!(
            "{} {} workout, done at {}. Exercises:",
            self.workout_type, self.targeted_muscles, self.workout_date
        );

        for exercise in &self.exercises {
            msg_string.push_str(format!(" {}", exercise).as_str())
        }

        write!(f, "{}", msg_string.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWorkoutError;

impl FromStr for Workout {
    type Err = ParseWorkoutError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('\n') {
            Some((workout_metadata, exercises_str)) => {
                let workout_metadata = workout_metadata
                    .split(':')
                    .map(|s| s.to_string())
                    .take(3)
                    .collect::<Vec<_>>();

                let [workout_date, workout_type, targeted_muscles] =
                    <[String; 3]>::try_from(workout_metadata).ok().unwrap();

                let exercises = exercises_str
                    .split('\n')
                    .filter_map(|ex_str: &str| -> Option<Exercise> {
                        ex_str.parse::<Exercise>().ok()
                    })
                    .collect::<Vec<_>>();

                Ok(Self {
                    workout_date,
                    workout_type,
                    targeted_muscles,
                    exercises,
                })
            }

            None => {
                print!("Failed to parse workout. Please try again.");
                Err(ParseWorkoutError)
            }
        }
    }
}
