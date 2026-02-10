use uuid::Uuid;

#[derive(Clone)]
pub enum Intensity {
    RIR(u8),
    RPE(u8),
}

#[derive(Clone)]
pub enum Weight {
    Lbs(f32),
    Kg(f32),
}

/// These are the sets that make up each exercise and the main tracking target
/// for progress.
#[derive(Clone)]
pub struct Set {
    pub id: Uuid,
    pub reps: u8,
    pub weight: Option<Weight>,
    pub intensity: Option<Intensity>,
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
    pub id: Uuid,
    pub exercise_name: String,
    pub equipment: String,
    pub sets: Vec<Set>,
}

impl Exercise {
    pub fn new(exercise_name: String, equipment: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            exercise_name,
            equipment,
            sets: Vec::new(),
        }
    }

    pub fn add_set(&mut self) -> Result<(), String> {
        todo!()
    }
}
