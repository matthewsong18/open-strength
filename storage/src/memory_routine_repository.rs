use domain::routine_repository::RoutineRepository;
use uuid::Uuid;

pub struct MemoryRoutineRepository {}

impl MemoryRoutineRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MemoryRoutineRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutineRepository for MemoryRoutineRepository {
    async fn get_all(&self) -> Result<Vec<domain::routine::Routine>, String> {
        todo!()
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<domain::routine::Routine>, String> {
        todo!()
    }

    async fn save(&self, routine: domain::routine::Routine) -> Result<(), String> {
        todo!()
    }
}
