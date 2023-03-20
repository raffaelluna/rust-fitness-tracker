use crate::command::error::ProcessorError;
use crate::model::workout_model::Workout;
use dotenv::dotenv;
use frankenstein::Message;
use frankenstein::{Api, Update, UpdateContent};
use serde_json::json;
use std::str::FromStr;

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

#[derive(Debug, Clone)]
pub struct UpdateProcessor {
    api: Api,
    update: Update,
    pub message: Message,
    command: Command,
    param: String,
}

impl UpdateProcessor {
    pub fn new(api: Api, update: Update) -> Result<Self, ProcessorError> {
        if let UpdateContent::Message(message) = update.content.clone() {
            if message.text.is_none() {
                println!("No message text found, please try again.");
                return Err(ProcessorError::MessageError(message));
            }

            let text = message.text.clone().unwrap();

            if let Some((command_candidate, param)) = text.split_once('\n') {
                let command = Command::from_str(command_candidate).unwrap();
                return Ok(Self {
                    api,
                    update,
                    message,
                    command,
                    param: param.to_string(),
                });
            }
            Err(ProcessorError::MessageError(message))
        } else {
            Err(ProcessorError::NoMessageError(()))
        }
    }
    pub fn run(&self) -> Result<String, ProcessorError> {
        match self.command {
            Command::RegisterExercise => {
                Ok(register_exercise(self.param.clone()))
            }
            Command::NewWorkout => Ok(new_workout(self.param.clone())),
            _ => Err(ProcessorError::InvalidCommandError(String::from(
                "Please, make sure to send a valid command.",
            ))),
        }
    }
}

fn new_workout(par: String) -> String {
    if let Ok(workout) = par.parse::<Workout>() {
        match validate_workout(workout) {
            Ok(workout) => {
                let msg_to_send = format!("{}", workout);
                println!("{:?}", msg_to_send);

                let Ok(db_handler) = DBHandler::new() else {
                    let msg_to_send = String::from(
                        "Could not load workout to database, please check and try again."
                    );
                    return msg_to_send;
                };

                if let Err(err) = db_handler.load_workout_to_db(&workout) {
                    let msg_to_send = format!(
                        "Could not load workout to database, please check and try again. Error: {}.", err
                    );
                    return msg_to_send;
                }
                //
                // if let Some(last_workout) = get_last_workout(workout.workout_type) {
                //      show_differences(&workout, last_workout);
                // }

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

fn register_exercise(par: String) -> String {
    format!("Command: InsertExercise\nPar: {:?}", par)
}

// TODO
fn validate_workout(workout: Workout) -> Result<Workout, ()> {
    // ver se nao há dois treinos no mesmo dia
    Ok(workout)
}

#[derive(Debug, Clone)]
pub struct DBHandler {
    api_url: String,
}

impl DBHandler {
    fn new() -> Result<Self, ()> {
        dotenv().ok();
        let api_url = std::env::var("API_URL").expect("API_URL must be set.");

        Ok(Self { api_url })
    }
    fn load_workout_to_db(
        &self,
        workout: &Workout,
    ) -> Result<(), Box<ureq::Error>> {
        match ureq::post(&self.api_url).send_json(json!(&workout)) {
            Ok(response) => {
                println!(
                    "Successfully sent the workout. Got response:\n{}",
                    response.into_string().unwrap()
                );
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    #[allow(dead_code)]
    fn get_last_workout(&self, workout_type: String) -> Option<Workout> {
        match ureq::get(&self.api_url).send_string(workout_type.as_str()) {
            Ok(response) => response.into_json::<Workout>().ok(),
            Err(_) => None,
        }
    }
}
