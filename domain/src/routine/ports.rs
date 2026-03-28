use crate::routine::models::root::RoutineName;

use super::models::root::{
    AddExerciseToRoutineCommand, AddExerciseToRoutineError, CreateRoutineCommand,
    CreateRoutineError, RenameRoutineCommand, RenameRoutineError, Routine,
};

use thiserror::Error;
use uuid::Uuid;

pub trait RoutineService: Clone + Send + Sync + 'static {
    fn create_routine(
        &self,
        req: &CreateRoutineCommand,
    ) -> impl Future<Output = Result<Routine, CreateRoutineError>> + Send;

    fn rename_routine(
        &self,
        req: &RenameRoutineCommand,
    ) -> impl Future<Output = Result<Routine, RenameRoutineError>> + Send;

    fn add_exercise(
        &self,
        req: &AddExerciseToRoutineCommand,
    ) -> impl Future<Output = Result<Routine, AddExerciseToRoutineError>> + Send;
}

pub trait RoutineRepository: Clone + Send + Sync + 'static {
    fn exists_by_name(
        &self,
        name: &RoutineName,
    ) -> impl Future<Output = Result<bool, RoutineRepositoryError>> + Send;
    fn get_all(&self) -> impl Future<Output = Result<Vec<Routine>, RoutineRepositoryError>> + Send;
    fn get_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Routine>, RoutineRepositoryError>> + Send;
    fn save(
        &self,
        routine: &Routine,
    ) -> impl Future<Output = Result<(), RoutineRepositoryError>> + Send;
}

#[derive(Debug, Error)]
pub enum RoutineRepositoryError {
    #[error("An internal storage error occurred: {0}")]
    Internal(String),
}
