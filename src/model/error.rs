use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseWorkoutError {
    #[error("Workout string not provided.")]
    WorkoutNotFoundError(),
    #[error("Could not parse workout metadata.")]
    WorkoutMetadataError(),
}
