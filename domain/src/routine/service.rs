use anyhow::anyhow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{
    models::{
        exercise::Exercise,
        root::{
            CreateRoutineError, CreateRoutineRequest, RenameRoutineError, RenameRoutineRequest,
            Routine, RoutineName,
        },
    },
    ports::{RoutineRepository, RoutineService},
};

#[derive(Debug, Clone)]
pub struct Service<R: RoutineRepository> {
    repo: R,
}

impl<R> Service<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> RoutineService for Service<R>
where
    R: RoutineRepository,
{
    async fn create_routine(
        &self,
        req: &CreateRoutineRequest,
    ) -> Result<Routine, CreateRoutineError> {
        let id: Uuid = Uuid::now_v7();
        let name: RoutineName = req.name().clone();
        let created_at: DateTime<Utc> = Utc::now();
        let exercises: Vec<Exercise> = Vec::new();
        let routine: Routine = Routine::new(id, name, created_at, exercises);
        match self.repo.save(&routine).await {
            Ok(_) => Ok(routine),
            Err(err) => Err(CreateRoutineError::Unknown(anyhow!(err))),
        }
    }

    async fn rename_routine(
        &self,
        req: &RenameRoutineRequest,
    ) -> Result<Routine, RenameRoutineError> {
        let mut routine = self
            .repo
            .get_by_id(*req.target_id())
            .await
            .map_err(|e| RenameRoutineError::Unknown(anyhow!(e)))?
            .ok_or_else(|| RenameRoutineError::NotFound(*req.target_id()))?;

        routine.set_name(req.new_name().clone());

        Ok(routine)
    }
}
