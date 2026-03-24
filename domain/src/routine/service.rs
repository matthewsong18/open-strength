use super::models::root::{CreateRoutineError, CreateRoutineRequest, Routine};
use super::ports::{RoutineRepository, RoutineService};

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
}
