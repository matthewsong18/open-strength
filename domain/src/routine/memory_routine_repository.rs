use super::models::exercise::Exercise;
use super::models::root::{CreateRoutineError, CreateRoutineRequest, Routine, RoutineName};
use super::ports::{RoutineRepository, RoutineRepositoryError};

use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MemoryRoutineRepository {
    routine_storage: Arc<Mutex<Vec<Routine>>>,
}

impl MemoryRoutineRepository {
    pub fn new() -> Self {
        Self {
            routine_storage: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MemoryRoutineRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutineRepository for MemoryRoutineRepository {
    async fn get_all(&self) -> Result<Vec<Routine>, RoutineRepositoryError> {
        let storage = self
            .routine_storage
            .lock()
            .map_err(|_| RoutineRepositoryError::Internal("Lock poisoned".to_string()))?;
        Ok(storage.clone())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Routine>, RoutineRepositoryError> {
        let storage = self
            .routine_storage
            .lock()
            .map_err(|_| RoutineRepositoryError::Internal("Lock poisoned".to_string()))?;
        let routine: Option<Routine> = storage.iter().find(|routine| *routine.id() == id).cloned();
        Ok(routine)
    }

    async fn create_routine(
        &self,
        req: &CreateRoutineRequest,
    ) -> Result<Routine, CreateRoutineError> {
        let mut storage = self
            .routine_storage
            .lock()
            .map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        let id: Uuid = Uuid::now_v7();
        let name: RoutineName = req.name().clone();
        let created_at: DateTime<Utc> = Utc::now();
        let exercises: Vec<Exercise> = Vec::new();
        let routine: Routine = Routine::new(id, name, created_at, exercises);
        storage.push(routine.clone());
        Ok(routine)
    }

    async fn save(&self, routine: &Routine) -> Result<(), RoutineRepositoryError> {
        let mut storage = self
            .routine_storage
            .lock()
            .map_err(|_| RoutineRepositoryError::Internal("Lock poisoned".to_string()))?;

        if let Some(existing_routine) = storage.iter_mut().find(|r| r.id() == routine.id()) {
            *existing_routine = routine.clone();
        } else {
            storage.push(routine.clone());
        }

        Ok(())
    }
}
