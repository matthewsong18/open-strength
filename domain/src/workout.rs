use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{common::Exercise, routine::Routine};

pub struct Workout {
    id: Uuid,
    source_routine_id: Option<Uuid>,
    name: String,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    exercises: Vec<Exercise>,
}

impl Workout {
    pub fn new_freestyle() -> Self {
        Self {
            name: "Freestyle Workout".to_string(),
            ..Default::default()
        }
    }

    pub fn from_routine(routine: &Routine) -> Self {
        Self {
            source_routine_id: Some(routine.id()),
            name: routine.name().to_string(),
            exercises: routine.get_exercises().to_vec(),
            ..Default::default()
        }
    }

    // Getters

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn source_routine_id(&self) -> Option<Uuid> {
        self.source_routine_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }

    pub fn get_exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    // Setters

    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), String> {
        self.name = name.into();
        Ok(())
    }

    pub fn set_completed(&mut self) {
        self.completed_at = Some(Utc::now())
    }
}

impl Default for Workout {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            source_routine_id: None,
            name: "Untitled Workout".into(),
            created_at: Utc::now(),
            completed_at: None,
            exercises: Vec::new(),
        }
    }
}
