use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::ports::{WorkoutRepository, WorkoutRepositoryError};
use super::workout::Workout;

#[derive(Debug, Clone)]
pub struct MemoryWorkoutRepository {
    storage: Arc<Mutex<Vec<Workout>>>,
}

impl MemoryWorkoutRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MemoryWorkoutRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkoutRepository for MemoryWorkoutRepository {
    async fn get_all(&self) -> Result<Vec<Workout>, WorkoutRepositoryError> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| WorkoutRepositoryError::Internal("Lock poisoned".to_string()))?;
        Ok(storage.clone())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Workout>, WorkoutRepositoryError> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| WorkoutRepositoryError::Internal("Lock poisoned".to_string()))?;
        Ok(storage.iter().find(|w| w.id() == id).cloned())
    }

    async fn save(&self, workout: &Workout) -> Result<(), WorkoutRepositoryError> {
        let mut storage = self
            .storage
            .lock()
            .map_err(|_| WorkoutRepositoryError::Internal("Lock poisoned".to_string()))?;

        if let Some(existing) = storage.iter_mut().find(|w| w.id() == workout.id()) {
            *existing = workout.clone();
        } else {
            storage.push(workout.clone());
        }

        Ok(())
    }
}
