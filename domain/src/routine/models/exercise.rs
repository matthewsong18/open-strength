use super::set::Set;

use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

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

        set.set_reps(new_reps);

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
