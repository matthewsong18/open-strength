use uuid::Uuid;

pub enum Intensity {
    RIR(u8),
    RPE(u8),
}

pub enum Weight {
    Lbs(f32),
    Kg(f32),
}

/// These are the sets that make up each exercise and the main tracking target
/// for progress.
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

    pub fn get_reps(&self) -> u8 {
        self.reps
    }
}

/// These are the individual exercises that compose a workout.
pub struct Exercise {
    id: Uuid,
    name: String,
    equipment: String,
    sets: Vec<Set>,
}

impl Exercise {
    pub fn new(exercise_name: String, equipment: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: exercise_name,
            equipment,
            sets: Vec::new(),
        }
    }

    pub fn add_set(&mut self, reps: u8) {
        self.sets.push(Set::new(reps));
    }

    pub fn with_sets(mut self, sets: u8, reps: u8) -> Self {
        for _ in 0..sets {
            self.add_set(reps);
        }
        self
    }

    pub fn update_set_reps(&mut self, set_index: usize, new_reps: u8) -> Result<(), String> {
        let set = self
            .sets
            .get_mut(set_index)
            .ok_or("Set index out of bounds".to_string())?;

        set.reps = new_reps;

        Ok(())
    }

    pub fn get_sets(&self) -> &[Set] {
        &self.sets
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }
}
