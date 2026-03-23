use super::Routine;
use super::model::{CreateRoutineError, CreateRoutineRequest};
use uuid::Uuid;

pub trait RoutineService: Clone + Send + Sync + 'static {
    fn create_routine(
        &self,
        req: &CreateRoutineRequest,
    ) -> impl Future<Output = Result<Routine, CreateRoutineError>> + Send;
}

pub trait RoutineRepository: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Routine>, String>> + Send;
    fn get_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Routine>, String>> + Send;
    fn create_routine(
        &self,
        req: &CreateRoutineRequest,
    ) -> impl Future<Output = Result<Routine, CreateRoutineError>> + Send;
}
