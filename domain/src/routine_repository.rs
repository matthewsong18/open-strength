use crate::routine::Routine;
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait RoutineRepository {
    async fn get_all(&self) -> Result<Vec<Routine>, String>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Routine>, String>;
    async fn save(&self, routine: Routine) -> Result<(), String>;
}
