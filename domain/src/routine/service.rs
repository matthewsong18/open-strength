use thiserror::Error;
use uuid::Uuid;

use super::{
    models::{
        exercise::{
            EquipmentName, EquipmentNameEmptyError, Exercise, ExerciseName, ExerciseNameEmptyError,
        },
        root::{Routine, RoutineName, RoutineNameEmptyError},
    },
    ports::{RoutineRepository, RoutineRepositoryError},
};

#[derive(Debug, Clone)]
pub struct RoutineService<R: RoutineRepository> {
    repo: R,
}

impl<R> RoutineService<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_routine(
        &self,
        cmd: &CreateRoutineCommand,
    ) -> Result<Routine, CreateRoutineError> {
        let routine_name = RoutineName::try_from(cmd.name.clone())?;
        let routine: Routine = Routine::new(routine_name.clone());

        let exists_by_name = self.repo.exists_by_name(&routine_name).await?;

        if exists_by_name {
            return Err(CreateRoutineError::Duplicate(routine_name));
        }

        self.repo.save(&routine).await?;

        Ok(routine)
    }

    pub async fn rename_routine(
        &self,
        cmd: &RenameRoutineCommand,
    ) -> Result<Routine, RenameRoutineError> {
        let mut routine = self
            .repo
            .get_by_id(cmd.target_id)
            .await?
            .ok_or(RenameRoutineError::NotFound(cmd.target_id))?;

        let routine_name = RoutineName::try_from(cmd.new_name.clone())?;
        let exists_by_name = self.repo.exists_by_name(&routine_name).await?;

        if exists_by_name {
            return Err(RenameRoutineError::Duplicate(routine_name));
        }

        routine.set_name(routine_name);

        Ok(routine)
    }

    pub async fn add_exercise(
        &self,
        cmd: &AddExerciseToRoutineCommand,
    ) -> Result<Routine, AddExerciseToRoutineError> {
        let mut routine = self
            .repo
            .get_by_id(cmd.target_id)
            .await?
            .ok_or(AddExerciseToRoutineError::NotFound(cmd.target_id))?;

        let exercise_name: ExerciseName = ExerciseName::new(&cmd.exercise_name)?;
        let equipment_name: Option<EquipmentName> = match &cmd.equipment_name {
            Some(raw_equipment_name) => {
                let equipment_name = EquipmentName::new(raw_equipment_name)?;
                Some(equipment_name)
            }
            None => None,
        };

        let sets = cmd.number_of_sets.unwrap_or(3u8);
        let reps = cmd.number_of_reps.unwrap_or(10u8);

        let exercise = Exercise::new(exercise_name, equipment_name).with_sets(sets, reps);

        routine.add_exercise(exercise);

        Ok(routine)
    }

    pub async fn rename_exercise(
        &self,
        cmd: &RenameExerciseCommand,
    ) -> Result<Routine, RenameExerciseError> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateRoutineCommand {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum CreateRoutineError {
    #[error(transparent)]
    Validation(#[from] RoutineNameEmptyError),

    #[error("routine with name {0} already exists")]
    Duplicate(RoutineName),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenameRoutineCommand {
    pub new_name: String,
    pub target_id: Uuid,
}

#[derive(Debug, Error)]
pub enum RenameRoutineError {
    #[error(transparent)]
    Validation(#[from] RoutineNameEmptyError),

    #[error("routine with id {0} could not be found")]
    NotFound(Uuid),

    #[error("routine with name {0} already exists")]
    Duplicate(RoutineName),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddExerciseToRoutineCommand {
    pub target_id: Uuid,
    pub exercise_name: String,
    pub equipment_name: Option<String>,
    pub number_of_sets: Option<u8>,
    pub number_of_reps: Option<u8>,
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
    pub routine_id: Uuid,
    pub exercise_id: Uuid,
    pub new_name: String,
}

#[derive(Debug, Error)]
pub enum RenameExerciseError {
    #[error(transparent)]
    ExerciseValidation(#[from] ExerciseNameEmptyError),

    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("exercise with id {0} could not be found")]
    ExerciseNotFound(Uuid),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}
