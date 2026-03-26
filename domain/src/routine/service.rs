use super::{
    models::root::{
        CreateRoutineError, CreateRoutineRequest, RenameRoutineError, RenameRoutineRequest, Routine,
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
        self.repo.create_routine(req).await
    }

    async fn rename_routine(
        &self,
        req: &RenameRoutineRequest,
    ) -> Result<Routine, RenameRoutineError> {
        let mut routine = self
            .repo
            .get_by_id(*req.target_id())
            .await
            .map_err(|e| RenameRoutineError::Unknown(anyhow::anyhow!(e)))?
            .ok_or_else(|| RenameRoutineError::NotFound(*req.target_id()))?;

        routine.set_name(req.new_name().clone());

        Ok(routine)
    }
}
