use crate::routine::models::root::RoutineError;

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
    pub(crate) fn new(id: Uuid, name: ExerciseName, equipment: Option<EquipmentName>) -> Self {
        Self {
            id,
            name,
            equipment,
            sets: Vec::<Set>::new(),
        }
    }

    // Getters

    pub fn id(&self) -> Uuid {
        self.id
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

    pub(crate) fn set_name(&mut self, new_name: ExerciseName) {
        self.name = new_name
    }

    pub(crate) fn set_equipment(&mut self, new_equipment: EquipmentName) {
        self.equipment = Some(new_equipment)
    }

    pub(crate) fn add_set(&mut self, new_set: Set) {
        self.sets.push(new_set);
    }

    pub(crate) fn with_sets(mut self, sets: u8, reps: u8) -> Self {
        for _ in 0..sets {
            let new_set = Set::new(Uuid::now_v7(), reps);
            self.add_set(new_set);
        }
        self
    }

    pub(crate) fn update_set_reps(
        &mut self,
        set_index: usize,
        new_reps: u8,
    ) -> Result<(), RoutineError> {
        let set = self
            .sets
            .get_mut(set_index)
            .ok_or(RoutineError::SetOutOfBounds(set_index))?;

        set.set_reps(new_reps);

        Ok(())
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
        let exercise_id = Uuid::now_v7();
        let exercise_name: ExerciseName = ExerciseName::new("Chest Press").unwrap();
        let mut exercise = Exercise::new(exercise_id, exercise_name, None);

        let start_count = exercise.sets().len();
        assert_eq!(0, start_count);

        let new_set = Set::new(Uuid::now_v7(), 10);
        exercise.add_set(new_set);

        let end_count = exercise.sets().len();
        assert_eq!(1, end_count);
    }
}
