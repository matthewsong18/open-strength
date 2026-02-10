use crate::common::Exercise;

use uuid::Uuid;

/// The aggregate root and core of the domain is tracking repeatable workouts.
/// As such, this contains all the information necessary for a given workout.
pub struct Routine {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    exercises: Vec<Exercise>,
}

impl Routine {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: chrono::Utc::now(),
            exercises: Vec::new(),
        }
    }

    pub fn add_exercise(
        &mut self,
        exercise_name: String,
        equipment: String,
        default_sets: u8,
        default_reps: u8,
    ) -> Result<(), String> {
        let mut new_exercise = Exercise::new(exercise_name, equipment);
        for _ in 0..default_sets {
            new_exercise.add_set_with_reps(default_reps);
        }
        self.exercises.push(new_exercise);

        Ok(())
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
        let exercise = self
            .exercises
            .get_mut(exercise_index)
            .ok_or("Exercise index out of bounds".to_string())?;

        exercise.update_set_reps(set_index, new_reps)?;

        Ok(())
    }
}

impl Default for Routine {
    fn default() -> Self {
        Self::new()
    }
}
