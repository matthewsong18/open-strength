use std::sync::Mutex;

use domain::{routine::Routine, routine_repository::RoutineRepository};
use uuid::Uuid;

pub struct MemoryRoutineRepository {
    routine_storage: Mutex<Vec<Routine>>,
}

impl MemoryRoutineRepository {
    pub fn new() -> Self {
        Self {
            routine_storage: Mutex::new(Vec::new()),
        }
    }
}

impl Default for MemoryRoutineRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutineRepository for MemoryRoutineRepository {
    async fn get_all(&self) -> Result<Vec<Routine>, String> {
        let storage = self.routine_storage.lock().map_err(|_| "Lock poisoned")?;
        Ok(storage.clone())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Routine>, String> {
        let storage = self.routine_storage.lock().map_err(|_| "Lock poisoned")?;
        let routine: Option<Routine> = storage.iter().find(|routine| routine.id() == id).cloned();
        Ok(routine)
    }

    async fn save(&self, routine: Routine) -> Result<(), String> {
        let mut storage = self.routine_storage.lock().map_err(|_| "Lock poisoned")?;
        storage.push(routine);
        Ok(())
    }
}
