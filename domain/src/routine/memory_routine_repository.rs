use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::models::root::Routine;
use super::ports::{RoutineRepository, RoutineRepositoryError};

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
        let routine: Option<Routine> = storage.iter().find(|routine| routine.id() == id).cloned();
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

    async fn exists_by_name(
        &self,
        name: &super::models::root::RoutineName,
    ) -> Result<bool, RoutineRepositoryError> {
        let mut storage = self
            .routine_storage
            .lock()
            .map_err(|_| RoutineRepositoryError::Internal("Lock poisoned".to_string()))?;

        match storage.iter_mut().find(|r| r.name() == name) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
