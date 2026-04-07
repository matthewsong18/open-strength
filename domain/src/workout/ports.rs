use super::workout::Workout;

use thiserror::Error;
use uuid::Uuid;

pub trait WorkoutRepository: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Workout>, WorkoutRepositoryError>> + Send;
    fn get_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Workout>, WorkoutRepositoryError>> + Send;
    fn save(
        &self,
        workout: &Workout,
    ) -> impl Future<Output = Result<(), WorkoutRepositoryError>> + Send;
}

#[derive(Debug, Error)]
pub enum WorkoutRepositoryError {
    #[error("An internal storage error occurred: {0}")]
    Internal(String),
}
