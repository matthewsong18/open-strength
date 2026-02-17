use crate::common::Exercise;

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The aggregate root and core of the domain is tracking repeatable workouts.
/// As such, this contains all the information necessary for a given workout.
pub struct Routine {
    id: Uuid,
    created_at: DateTime<Utc>,
    exercises: Vec<Exercise>,
}

impl Routine {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: Utc::now(),
            exercises: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    pub fn get_exercise(&self, exercise_index: usize) -> Option<&Exercise> {
        self.exercises.get(exercise_index)
    }

    pub fn exercise_count(&self) -> usize {
        self.exercises.len()
    }

    pub fn update_set_target_reps(
        &mut self,
        exercise_index: usize,
        set_index: usize,
        new_reps: u8,
    ) -> Result<(), String> {
        self.exercises
            .get_mut(exercise_index)
            .map(|exercise| exercise.update_set_reps(set_index, new_reps))
            .ok_or("Exercise index out of bounds".to_string())?
    }

    pub fn add_set_to_exercise(&mut self, exercise_id: Uuid, reps: u8) -> Result<(), String> {
        self.exercises
            .iter_mut()
            .find(|exercise| exercise.id() == exercise_id)
            .map(|exercise| {
                exercise.add_set(reps);
            })
            .ok_or_else(|| format!("Exercise with ID {} not found", exercise_id))
    }
}

impl Default for Routine {
    fn default() -> Self {
        Self::new()
    }
}
