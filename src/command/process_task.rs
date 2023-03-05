use std::convert::TryFrom;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Exercise {
    exercise_name: String,
    sets: i32,
    repetitions: i32,
    load: i32,
}

pub struct Workout {
    workout_date: String,
    workout_type: String,
    targeted_muscles: String,
    exercises: Vec<Exercise>,
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

pub fn parse_workout_from_str(s: &str) -> Option<Workout> {
    match s.split_once("\n") {
        Some((workout_metadata, exercises_str)) => {
            let workout_metadata = workout_metadata
                .split(":")
                .map(|s| s.to_string())
                .take(3)
                .collect::<Vec<_>>();

            let [workout_date, workout_type, targeted_muscles] =
                <[String; 3]>::try_from(workout_metadata).ok().unwrap();

            let exercises = exercises_str
                .split("\n")
                .map(|ex_str: &str| -> Option<Exercise> {
                    parse_exercise_from_str(ex_str).ok()
                })
                .flatten()
                .collect::<Vec<_>>();

            Some(Workout {
                workout_date,
                workout_type,
                targeted_muscles,
                exercises,
            })
        }

        None => {
            print!("Failed to parse workout. Please try again.");
            None
        }
    }
}

pub fn parse_exercise_from_str(s: &str) -> Result<Exercise, ParseIntError> {
    let mut exercise_data = s.split(":").take(4);

    let exercise_name = String::from(exercise_data.next().unwrap());
    let sets = exercise_data.next().unwrap().parse::<i32>()?;
    let repetitions = exercise_data.next().unwrap().parse::<i32>()?;
    let load = exercise_data.next().unwrap().parse::<i32>()?;

    Ok(Exercise {
        exercise_name,
        sets,
        repetitions,
        load,
    })
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    NewWorkout,
    RegisterExercise,
    UnknownCommand(String),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s.trim() {
            "/new_workout" => Command::NewWorkout,
            "/register_exercise" => Command::RegisterExercise,
            _ => Command::UnknownCommand(s.to_string()),
        };

        println!("{:?}", result);

        Ok(result)
    }
}

pub fn process(command: Command, par: &str) -> String {
    match command {
        Command::RegisterExercise => register_exercise(par),
        Command::NewWorkout => new_workout(par),
        _ => String::from("Please, make sure to send a valid command."),
    }
}

fn new_workout(par: &str) -> String {
    if let Some(workout) = parse_workout_from_str(par) {
        match validate_workout(workout) {
            Ok(workout) => {
                let msg_to_send = format!(
                    "Got a valid workout.\nWorkout date: {:?}\nWorkout type: {:?}\nTargeted muscles: {:?}\nExercises: {:?}",
                    workout.workout_date,
                    workout.workout_type,
                    workout.targeted_muscles,
                    workout.exercises
                );

                println!("{:?}", msg_to_send);

                // if let Err(err) = load_workout_to_db(&workout, url):
                //      let msg_to_send = format!("Could not load workout to database, please check and try again. Error: {}.", err)
                //      return msg_to_send; // usar o early return ()
                //
                // if let Some(last_workout) = get_last_workout(workout.workout_type, url):
                //      show_differences(&workout, last_workout);

                msg_to_send
            }
            _ => {
                let msg_to_send =
                    "The provided workout is not valid. Please try again."
                        .to_string();
                println!("{:?}", msg_to_send);
                msg_to_send
            }
        }
    } else {
        let msg_to_send = String::from(
            "Unable to parse the provided text to workout, please try again.",
        );
        println!("{:?}", msg_to_send);
        msg_to_send
    }
}

fn register_exercise(par: &str) -> String {
    format!("Command: InsertExercise\nPar: {:?}", par)
}

// TODO
fn validate_workout(workout: Workout) -> Result<Workout, ()> {
    Ok(workout)
}

#[allow(unused_variables)]
fn load_workout_to_db(workout: &Workout, url: &str) -> Result<(), ()> {
    // dotenv().ok()
    // let url = std::env::var("DB_URL").expect("DB_URL must be set.");
    // usar o operador ?
    // implementar erro
    Ok(())
}