use super::models::root::{
    CreateRoutineError, CreateRoutineRequest, RenameRoutineError, RenameRoutineRequest, Routine,
};

use thiserror::Error;
use uuid::Uuid;

pub trait RoutineService: Clone + Send + Sync + 'static {
    fn create_routine(
        &self,
        req: &CreateRoutineRequest,
    ) -> impl Future<Output = Result<Routine, CreateRoutineError>> + Send;

    fn rename_routine(
        &self,
        req: &RenameRoutineRequest,
    ) -> impl Future<Output = Result<Routine, RenameRoutineError>> + Send;
}

pub trait RoutineRepository: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Routine>, String>> + Send;
    fn get_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Routine>, String>> + Send;
    fn create_routine(
        &self,
        req: &CreateRoutineRequest,
    ) -> impl Future<Output = Result<Routine, CreateRoutineError>> + Send;
    fn save(
        &self,
        routine: &Routine,
    ) -> impl Future<Output = Result<(), RoutineRepositoryError>> + Send;
}

#[derive(Debug, Error)]
pub enum RoutineRepositoryError {
    #[error("Routine with ID {0} was not found")]
    NotFound(Uuid),

    #[error("A storage conflict occurred: {0}")]
    Conflict(String),

    #[error("An internal storage error occurred: {0}")]
    Internal(String),
}
