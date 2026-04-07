use thiserror::Error;
use uuid::Uuid;

use crate::routine::{
    models::{
        root::{Routine, RoutineError},
        set::Set,
    },
    ports::{RoutineRepository, RoutineRepositoryError},
};

#[derive(Debug, Clone)]
pub struct RoutineDetailingService<R: RoutineRepository> {
    repo: R,
}

impl<R> RoutineDetailingService<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn add_set(&self, cmd: &AddSetCommand) -> Result<Routine, AddSetError> {
        let mut routine: Routine = self
            .repo
            .get_by_id(cmd.routine_id)
            .await?
            .ok_or(AddSetError::RoutineNotFound(cmd.routine_id()))?;

        let new_set: Set = Set::new(cmd.new_set_id(), cmd.number_of_reps.unwrap_or(10u8));
        routine.add_set_to_exercise(cmd.exercise_id(), new_set)?;

        self.repo.save(&routine).await?;

        Ok(routine)
    }
}

#[derive(Clone, Debug)]
pub struct AddSetCommand {
    routine_id: Uuid,
    exercise_id: Uuid,
    new_set_id: Uuid,
    number_of_reps: Option<u8>,
}

impl AddSetCommand {
    pub fn new(routine_id: Uuid, exercise_id: Uuid) -> Self {
        Self {
            routine_id,
            exercise_id,
            new_set_id: Uuid::now_v7(),
            number_of_reps: None,
        }
    }

    pub fn with_reps(mut self, reps: u8) -> Self {
        self.number_of_reps = Some(reps);
        self
    }

    pub fn exercise_id(&self) -> Uuid {
        self.exercise_id
    }

    pub fn new_set_id(&self) -> Uuid {
        self.new_set_id
    }

    pub fn number_of_reps(&self) -> Option<u8> {
        self.number_of_reps
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }
}

#[derive(Debug, Error)]
pub enum AddSetError {
    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("domain rule violation: {0}")]
    Domain(#[from] RoutineError),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}
