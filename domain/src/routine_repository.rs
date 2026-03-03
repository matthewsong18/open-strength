use crate::routine::Routine;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RoutineRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Routine>, String>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Routine>, String>;
    async fn save(&self, routine: Routine) -> Result<(), String>;
}
