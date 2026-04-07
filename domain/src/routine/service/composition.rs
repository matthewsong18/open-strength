use thiserror::Error;
use uuid::Uuid;

use crate::routine::{
    models::{
        exercise::{
            EquipmentName, EquipmentNameEmptyError, Exercise, ExerciseName, ExerciseNameEmptyError,
        },
        root::{Routine, RoutineError},
    },
    ports::{RoutineRepository, RoutineRepositoryError},
};

#[derive(Debug, Clone)]
pub struct RoutineCompositionService<R: RoutineRepository> {
    repo: R,
}

impl<R> RoutineCompositionService<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn add_exercise(
        &self,
        cmd: &AddExerciseToRoutineCommand,
    ) -> Result<Routine, AddExerciseToRoutineError> {
        let exercise_id: Uuid = cmd.new_exercise_id();
        let exercise_name: ExerciseName = ExerciseName::new(cmd.exercise_name())?;
        let equipment_name: Option<EquipmentName> =
            cmd.equipment_name().map(EquipmentName::new).transpose()?;

        let mut exercise = Exercise::new(exercise_id, exercise_name, equipment_name);

        if let (Some(sets), Some(reps)) = (cmd.number_of_sets(), cmd.number_of_reps()) {
            exercise = exercise.with_sets(sets, reps);
        }

        let mut routine = self
            .repo
            .get_by_id(cmd.routine_id())
            .await?
            .ok_or(AddExerciseToRoutineError::NotFound(cmd.routine_id()))?;

        routine.add_exercise(exercise);

        self.repo.save(&routine).await?;

        Ok(routine)
    }

    pub async fn rename_exercise(
        &self,
        cmd: &RenameExerciseCommand,
    ) -> Result<Routine, RenameExerciseError> {
        let new_name = ExerciseName::new(&cmd.new_name)?;
        let exercise_id = cmd.exercise_id;

        let mut routine = self
            .repo
            .get_by_id(cmd.routine_id)
            .await?
            .ok_or(RenameExerciseError::RoutineNotFound(cmd.routine_id))?;

        routine.rename_exercise(exercise_id, new_name)?;

        self.repo.save(&routine).await?;

        Ok(routine)
    }

    pub async fn delete_exercise(
        &self,
        cmd: &DeleteExerciseCommand,
    ) -> Result<Routine, DeleteExerciseError> {
        let routine_id = cmd.routine_id();
        let exercise_id = cmd.exercise_id();

        let mut routine = self
            .repo
            .get_by_id(routine_id)
            .await?
            .ok_or(DeleteExerciseError::RoutineNotFound(routine_id))?;

        routine.delete_exercise(exercise_id)?;

        self.repo.save(&routine).await?;

        Ok(routine)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddExerciseToRoutineCommand {
    routine_id: Uuid,
    new_exercise_id: Uuid,
    exercise_name: String,
    equipment_name: Option<String>,
    number_of_sets: Option<u8>,
    number_of_reps: Option<u8>,
}

impl AddExerciseToRoutineCommand {
    pub fn new(routine_id: Uuid, exercise_name: impl Into<String>) -> Self {
        Self {
            routine_id,
            new_exercise_id: uuid::Uuid::now_v7(),
            exercise_name: exercise_name.into(),
            equipment_name: None,
            number_of_sets: None,
            number_of_reps: None,
        }
    }

    pub fn with_equipment(mut self, equipment: impl Into<String>) -> Self {
        self.equipment_name = Some(equipment.into());
        self
    }

    pub fn with_sets_and_reps(mut self, sets: u8, reps: u8) -> Self {
        self.number_of_sets = Some(sets);
        self.number_of_reps = Some(reps);
        self
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }

    pub fn new_exercise_id(&self) -> Uuid {
        self.new_exercise_id
    }

    pub fn exercise_name(&self) -> &str {
        &self.exercise_name
    }

    pub fn equipment_name(&self) -> Option<&str> {
        self.equipment_name.as_deref()
    }

    pub fn number_of_sets(&self) -> Option<u8> {
        self.number_of_sets
    }

    pub fn number_of_reps(&self) -> Option<u8> {
        self.number_of_reps
    }
}

#[derive(Debug, Error)]
pub enum AddExerciseToRoutineError {
    #[error(transparent)]
    ExerciseValidation(#[from] ExerciseNameEmptyError),

    #[error(transparent)]
    EquipmentValidation(#[from] EquipmentNameEmptyError),

    #[error("routine with id {0} could not be found")]
    NotFound(Uuid),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

#[derive(Clone, Debug)]
pub struct RenameExerciseCommand {
    routine_id: Uuid,
    exercise_id: Uuid,
    new_name: String,
}

impl RenameExerciseCommand {
    pub fn new(routine_id: Uuid, exercise_id: Uuid, new_name: impl Into<String>) -> Self {
        Self {
            routine_id,
            exercise_id,
            new_name: new_name.into(),
        }
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }

    pub fn exercise_id(&self) -> Uuid {
        self.exercise_id
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}

#[derive(Debug, Error)]
pub enum RenameExerciseError {
    #[error(transparent)]
    ExerciseValidation(#[from] ExerciseNameEmptyError),

    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("domain rule violation: {0}")]
    Domain(#[from] RoutineError),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

pub struct DeleteExerciseCommand {
    routine_id: Uuid,
    exercise_id: Uuid,
}

impl DeleteExerciseCommand {
    pub fn new(routine_id: Uuid, exercise_id: Uuid) -> Self {
        Self {
            routine_id,
            exercise_id,
        }
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }

    pub fn exercise_id(&self) -> Uuid {
        self.exercise_id
    }
}

#[derive(Debug, Error)]
pub enum DeleteExerciseError {
    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("domain rule violation: {0}")]
    Domain(#[from] RoutineError),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}
