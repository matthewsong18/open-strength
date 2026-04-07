use thiserror::Error;
use uuid::Uuid;

use crate::routine::{
    models::root::{Routine, RoutineName, RoutineNameEmptyError},
    ports::{RoutineRepository, RoutineRepositoryError},
};

#[derive(Debug, Clone)]
pub struct RoutineManagementService<R: RoutineRepository> {
    repo: R,
}

impl<R> RoutineManagementService<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_routine(
        &self,
        cmd: &CreateRoutineCommand,
    ) -> Result<Routine, CreateRoutineError> {
        let routine_name = RoutineName::try_from(cmd.name.clone())?;
        let routine: Routine = Routine::new(cmd.routine_id(), routine_name.clone());

        let exists_by_name = self.repo.exists_by_name(&routine_name).await?;
        if exists_by_name {
            return Err(CreateRoutineError::Duplicate(routine_name));
        }

        self.repo.save(&routine).await?;

        Ok(routine)
    }

    pub async fn rename_routine(
        &self,
        cmd: &RenameRoutineCommand,
    ) -> Result<Routine, RenameRoutineError> {
        let routine_name = RoutineName::try_from(cmd.new_name.clone())?;

        let exists_by_name = self.repo.exists_by_name(&routine_name).await?;
        if exists_by_name {
            return Err(RenameRoutineError::Duplicate(routine_name));
        }

        let mut routine = self
            .repo
            .get_by_id(cmd.routine_id)
            .await?
            .ok_or(RenameRoutineError::NotFound(cmd.routine_id))?;

        routine.set_name(routine_name);

        self.repo.save(&routine).await?;

        Ok(routine)
    }

    pub async fn get_routine(
        &self,
        query: &GetRoutineQuery,
    ) -> Result<Option<Routine>, GetRoutineError> {
        match query {
            GetRoutineQuery::ById(id) => Ok(self.repo.get_by_id(*id).await?),
            GetRoutineQuery::ByName(name) => {
                let routine_name = RoutineName::new(name)?;
                Ok(self.repo.get_by_name(&routine_name).await?)
            }
        }
    }

    pub async fn get_all_routines(&self) -> Result<Vec<Routine>, RoutineRepositoryError> {
        self.repo.get_all().await
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateRoutineCommand {
    routine_id: Uuid,
    name: String,
}

impl CreateRoutineCommand {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            routine_id: Uuid::now_v7(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }
}

#[derive(Debug, Error)]
pub enum CreateRoutineError {
    #[error(transparent)]
    Validation(#[from] RoutineNameEmptyError),

    #[error("routine with name {0} already exists")]
    Duplicate(RoutineName),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenameRoutineCommand {
    routine_id: Uuid,
    new_name: String,
}

impl RenameRoutineCommand {
    pub fn new(routine_id: Uuid, new_name: impl Into<String>) -> Self {
        Self {
            routine_id,
            new_name: new_name.into(),
        }
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }
}

#[derive(Debug, Error)]
pub enum RenameRoutineError {
    #[error(transparent)]
    Validation(#[from] RoutineNameEmptyError),

    #[error("routine with id {0} could not be found")]
    NotFound(Uuid),

    #[error("routine with name {0} already exists")]
    Duplicate(RoutineName),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetRoutineQuery {
    ById(Uuid),
    ByName(String),
}

#[derive(Debug, Error)]
pub enum GetRoutineError {
    #[error(transparent)]
    Validation(#[from] RoutineNameEmptyError),

    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}
