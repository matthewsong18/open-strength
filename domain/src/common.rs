use std::fmt::{Display, Formatter};

use thiserror::Error;
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
    name: ExerciseName,
    equipment: Option<EquipmentName>,
    sets: Vec<Set>,
}

impl Exercise {
    fn new(name: ExerciseName, equipment: Option<EquipmentName>) -> Self {
        Self {
            name,
            equipment,
            ..Default::default()
        }
    }

    // Getters

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> ExerciseName {
        self.name.clone()
    }

    pub fn equipment(&self) -> Option<EquipmentName> {
        self.equipment.clone()
    }

    pub fn sets(&self) -> &[Set] {
        &self.sets
    }

    // Setters

    pub fn set_name(&mut self, new_name: ExerciseName) {
        self.name = new_name
    }

    pub fn set_equipment(&mut self, new_equipment: EquipmentName) {
        self.equipment = Some(new_equipment)
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
}

impl Default for Exercise {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: ExerciseName::default(),
            equipment: None,
            sets: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExerciseName(String);

#[derive(Clone, Debug, Error)]
#[error("exercise name cannot be empty")]
pub struct ExerciseNameEmptyError;

impl ExerciseName {
    pub fn new(raw: &str) -> Result<Self, ExerciseNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(ExerciseNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Default for ExerciseName {
    fn default() -> Self {
        Self("Untitled Exercise".to_string())
    }
}

impl Display for ExerciseName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquipmentName(String);

#[derive(Clone, Debug, Error)]
#[error("equipment name cannot be empty")]
pub struct EquipmentNameEmptyError;

impl EquipmentName {
    pub fn new(raw: &str) -> Result<Self, EquipmentNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(EquipmentNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Default for EquipmentName {
    fn default() -> Self {
        Self("Untitled Equipment".to_string())
    }
}

impl Display for EquipmentName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_add_set() {
        let exercise_name: ExerciseName = ExerciseName::new("Chest Press").unwrap();
        let mut exercise = Exercise::new(exercise_name, None);

        let start_count = exercise.sets().len();
        assert_eq!(0, start_count);

        exercise.add_set(10);

        let end_count = exercise.sets().len();
        assert_eq!(1, end_count);
    }
}
