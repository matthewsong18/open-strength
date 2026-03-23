use crate::routine::ports::{RoutineRepository, RoutineService};

use super::Routine;
use super::model::{CreateRoutineError, CreateRoutineRequest};

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
