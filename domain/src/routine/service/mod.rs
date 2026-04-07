pub mod composition;
pub mod detailing;
pub mod management;

pub use composition::*;
pub use detailing::*;
pub use management::*;

use crate::routine::ports::{RoutineRepository, RoutineRepositoryError};
use crate::routine::models::root::Routine;

/// A facade service that combines routine management, composition, and detailing.
/// This maintains backward compatibility while delegating to specialized services.
#[derive(Debug, Clone)]
pub struct RoutineService<R: RoutineRepository> {
    management: RoutineManagementService<R>,
    composition: RoutineCompositionService<R>,
    detailing: RoutineDetailingService<R>,
}

impl<R> RoutineService<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self {
            management: RoutineManagementService::new(repo.clone()),
            composition: RoutineCompositionService::new(repo.clone()),
            detailing: RoutineDetailingService::new(repo),
        }
    }

    pub async fn create_routine(
        &self,
        cmd: &CreateRoutineCommand,
    ) -> Result<Routine, CreateRoutineError> {
        self.management.create_routine(cmd).await
    }

    pub async fn rename_routine(
        &self,
        cmd: &RenameRoutineCommand,
    ) -> Result<Routine, RenameRoutineError> {
        self.management.rename_routine(cmd).await
    }

    pub async fn get_routine(
        &self,
        query: &GetRoutineQuery,
    ) -> Result<Option<Routine>, GetRoutineError> {
        self.management.get_routine(query).await
    }

    pub async fn get_all_routines(&self) -> Result<Vec<Routine>, RoutineRepositoryError> {
        self.management.get_all_routines().await
    }

    pub async fn add_exercise(
        &self,
        cmd: &AddExerciseToRoutineCommand,
    ) -> Result<Routine, AddExerciseToRoutineError> {
        self.composition.add_exercise(cmd).await
    }

    pub async fn rename_exercise(
        &self,
        cmd: &RenameExerciseCommand,
    ) -> Result<Routine, RenameExerciseError> {
        self.composition.rename_exercise(cmd).await
    }

    pub async fn delete_exercise(
        &self,
        cmd: &DeleteExerciseCommand,
    ) -> Result<Routine, DeleteExerciseError> {
        self.composition.delete_exercise(cmd).await
    }

    pub async fn add_set(&self, cmd: &AddSetCommand) -> Result<Routine, AddSetError> {
        self.detailing.add_set(cmd).await
    }
}
