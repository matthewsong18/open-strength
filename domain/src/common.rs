use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intensity(u8);

impl Intensity {
    /// Creates an Intensity from Rate of Perceived Exertion (RPE).
    pub fn from_rpe(rpe: u8) -> Self {
        Intensity(rpe)
    }

    /// Creates an Intensity from Reps in Reserve (RIR).
    /// Assumes a standard 10-point scale (RPE = 10 - RIR).
    pub fn from_rir(rir: u8) -> Self {
        // saturating_sub prevents integer underflow panics if rir > 10
        Intensity(10u8.saturating_sub(rir))
    }

    /// Returns the intensity formatted as RPE.
    pub fn as_rpe(&self) -> u8 {
        self.0
    }

    /// Returns the intensity formatted as RIR.
    pub fn as_rir(&self) -> u8 {
        10u8.saturating_sub(self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weight(u32);

impl Weight {
    pub fn from_kg(kg: f32) -> Self {
        Weight((kg * 1000.0).round() as u32)
    }

    pub fn from_lbs(lbs: f32) -> Self {
        Weight((lbs * 453.592).round() as u32)
    }

    /// Returns the rounded kg weight as a whole integer or `.5`
    pub fn as_kg(&self) -> f32 {
        let exact_kg = self.0 as f32 / 1000.0;
        (exact_kg * 2.0).round() / 2.0
    }

    /// Returns the rounded lbs weight as a whole integer
    pub fn as_lbs(&self) -> f32 {
        let exact_lbs = self.0 as f32 / 453.592;
        exact_lbs.round()
    }
}

/// These are the sets that make up each exercise and the main tracking target
/// for progress.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn weight(&self) -> Option<Weight> {
        self.weight
    }

    pub fn intensity(&self) -> Option<Intensity> {
        self.intensity
    }

    pub fn reps(&self) -> u8 {
        self.reps
    }
}

/// These are the individual exercises that compose a workout.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Exercise {
    id: Uuid,
    name: String,
    equipment: String,
    sets: Vec<Set>,
}

impl Exercise {
    pub fn new(exercise_name: impl Into<String>, equipment: impl Into<String>) -> Self {
        Self {
            name: exercise_name.into(),
            equipment: equipment.into(),
            ..Default::default()
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

    pub fn set_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into()
    }

    pub fn equipment(&self) -> &str {
        self.equipment.as_ref()
    }

    pub fn set_equipment(&mut self, new_equipment: impl Into<String>) {
        self.equipment = new_equipment.into()
    }
}

impl Default for Exercise {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: "Untitled Exercise".into(),
            equipment: "None".into(),
            sets: Vec::new(),
        }
    }
}
