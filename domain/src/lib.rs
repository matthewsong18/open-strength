use uuid::Uuid;

#[derive(Clone)]
pub enum Intensity {
    RPE(u8),
    RIR(u8),
}

#[derive(Clone)]
pub enum Weight {
    Lbs(f32),
    Kg(f32),
}

type Equipment = String;

/// These are the sets that make up each exercise and the main tracking target
/// for progress.
#[derive(Clone)]
pub struct Set {
    id: Uuid,
    reps: u8,
    weight: Option<Weight>,
    intensity: Option<Intensity>,
}

impl Set {
    pub fn new(reps: u8) -> Self {
        Self {
            id: Uuid::now_v7(),
            reps,
            weight: None,
            intensity: None,
        }
    }
}

/// These are the individual exercises that compose a workout.
#[derive(Clone)]
pub struct Exercise {
    id: Uuid,
    exercise_name: String,
    equipment: Equipment,
    sets: Vec<Set>,
}

impl Exercise {
    pub fn new(exercise_name: String, equipment: Equipment) -> Self {
        Self {
            id: Uuid::now_v7(),
            exercise_name,
            equipment,
            sets: Vec::new(),
        }
    }
}

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
        equipment: Equipment,
        default_sets: u8,
        default_reps: u8,
    ) -> Result<(), String> {
        let mut new_exercise = Exercise::new(exercise_name, equipment);
        for _ in 0..default_sets {
            new_exercise.sets.push(Set::new(default_reps))
        }
        self.exercises.push(new_exercise);

        Ok(())
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

        let set = exercise
            .sets
            .get_mut(set_index)
            .ok_or("Set index out of bounds".to_string())?;

        set.reps = new_reps;

        Ok(())
    }
}

impl Default for Routine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_workout() {
        let workout = Routine::new();
        assert_eq!(workout.exercise_count(), 0);
    }

    #[test]
    fn add_exercise() {
        let mut workout = Routine::new();
        let result =
            workout.add_exercise("Chest Press".to_string(), "Bench Press".to_string(), 3, 10);
        assert_eq!(Ok(()), result);
        assert_eq!(workout.exercise_count(), 1);
        assert_eq!(workout.exercises[0].sets.len(), 3);
    }

    #[test]
    fn update_the_target_reps_of_a_set() {
        let mut workout = Routine::new();
        workout
            .add_exercise("Chest Press".to_string(), "Bench press".to_string(), 3, 10)
            .unwrap();

        workout.update_set_target_reps(0, 0, 7).unwrap();

        assert_eq!(workout.exercises[0].sets[0].reps, 7);
    }
}
