use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

use crate::routine::models::exercise::ExerciseName;

use super::{exercise::Exercise, set::Set};

/// The aggregate root and core of the domain is tracking repeatable workouts.
/// As such, this contains all the information necessary for a given workout.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Routine {
    id: Uuid,
    name: RoutineName,
    created_at: DateTime<Utc>,
    exercises: Vec<Exercise>,
}

impl Routine {
    pub fn new(name: RoutineName) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            created_at: Utc::now(),
            exercises: Vec::<Exercise>::new(),
        }
    }

    pub fn rebuild(
        id: Uuid,
        name: RoutineName,
        created_at: DateTime<Utc>,
        exercises: Vec<Exercise>,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            exercises,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &RoutineName {
        &self.name
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn get_exercise(&self, exercise_id: Uuid) -> Option<&Exercise> {
        self.exercises.iter().find(|e| *e.id() == exercise_id)
    }

    pub fn get_exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    pub fn exercise_count(&self) -> usize {
        self.exercises.len()
    }

    // Setters

    pub(crate) fn set_name(&mut self, name: RoutineName) {
        self.name = name;
    }

    pub(crate) fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    pub(crate) fn update_set_target_reps(
        &mut self,
        exercise_index: usize,
        set_index: usize,
        new_reps: u8,
    ) -> Result<(), RoutineError> {
        self.exercises
            .get_mut(exercise_index)
            .map(|exercise| exercise.update_set_reps(set_index, new_reps))
            .ok_or(RoutineError::SetOutOfBounds(set_index))?
    }

    pub(crate) fn add_set_to_exercise(
        &mut self,
        exercise_id: Uuid,
        new_set: Set,
    ) -> Result<(), RoutineError> {
        self.exercises
            .iter_mut()
            .find(|exercise| *exercise.id() == exercise_id)
            .map(|exercise| {
                exercise.add_set(new_set);
            })
            .ok_or(RoutineError::ExerciseNotFound(exercise_id))
    }

    pub(crate) fn rename_exercise(
        &mut self,
        exercise_id: Uuid,
        new_name: ExerciseName,
    ) -> Result<(), RoutineError> {
        self.exercises
            .iter_mut()
            .find(|exercise| *exercise.id() == exercise_id)
            .map(|exercise| exercise.set_name(new_name))
            .ok_or(RoutineError::ExerciseNotFound(exercise_id))
    }
}

#[derive(Debug, Error)]
pub enum RoutineError {
    #[error("exercise with id {0} not found in this routine")]
    ExerciseNotFound(Uuid),

    #[error("set index {0} is out of bounds for the exercise")]
    SetOutOfBounds(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoutineName(String);

#[derive(Clone, Debug, Error)]
#[error("routine name cannot be empty")]
pub struct RoutineNameEmptyError;

impl RoutineName {
    pub fn new(raw: &str) -> Result<Self, RoutineNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(RoutineNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl TryFrom<String> for RoutineName {
    type Error = RoutineNameEmptyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        RoutineName::new(&value)
    }
}

impl TryFrom<&str> for RoutineName {
    type Error = RoutineNameEmptyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        RoutineName::new(value)
    }
}

impl Display for RoutineName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
