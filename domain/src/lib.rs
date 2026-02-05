use chrono;
use uuid::Uuid;

pub enum Intensity {
    RPE(u8),
    RIR(u8),
}

pub enum Weight {
    Lbs(f32),
    Kg(f32),
}

type Equipment = String;

/// These are the sets that make up each exercise and the main tracking target
/// for progress.
pub struct Set {
    id: Uuid,
    reps: u32,
    weight: Weight,
    intensity: Option<Intensity>,
}

/// These are the individual exercises that compose a workout.
pub struct WorkoutExercise {
    id: Uuid,
    exercise_name: String,
    equipment: Equipment,
    sets: Vec<Set>,
}

/// The aggregate root and core of the domain is tracking repeatable workouts.
/// As such, this contains all the information necessary for a given workout.
pub struct Workout {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    exercises: Vec<WorkoutExercise>,
}

impl Workout {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: chrono::Utc::now(),
            exercises: Vec::new(),
        }
    }

    pub fn exercise_count(&self) -> usize {
        self.exercises.len()
    }

    pub fn add_exercise(
        &mut self,
        exercise_name: String,
        equipment: Equipment,
    ) -> Result<(), String> {
        Err("Unimplemented".to_string())
    }
}

impl Default for Workout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_workout() {
        let workout = Workout::new();
        assert_eq!(workout.exercise_count(), 0);
    }

    #[test]
    fn add_exercise() {
        let mut workout = Workout::new();
        let result = workout.add_exercise("Chest Press".to_string(), "Bench Press".to_string());
        assert_eq!(Ok(()), result);
        assert_eq!(workout.exercise_count(), 1);
    }
}
