use thiserror::Error;
use uuid::Uuid;

use super::{
    models::{
        exercise::{
            EquipmentName, EquipmentNameEmptyError, Exercise, ExerciseName, ExerciseNameEmptyError,
        },
        root::{Routine, RoutineError, RoutineName, RoutineNameEmptyError},
        set::Set,
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
        let routine: Routine = Routine::new(cmd.routine_id(), routine_name.clone());

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
        let routine_name = RoutineName::try_from(cmd.new_name.clone())?;

        let exists_by_name = self.repo.exists_by_name(&routine_name).await?;
        if exists_by_name {
            return Err(RenameRoutineError::Duplicate(routine_name));
        }

        let mut routine = self
            .repo
            .get_by_id(cmd.routine_id)
            .await?
            .ok_or(RenameRoutineError::NotFound(cmd.routine_id))?;

        routine.set_name(routine_name);

        self.repo.save(&routine).await?;

        Ok(routine)
    }

    pub async fn get_routine(
        &self,
        query: &GetRoutineQuery,
    ) -> Result<Option<Routine>, GetRoutineError> {
        match query {
            GetRoutineQuery::ById(id) => Ok(self.repo.get_by_id(*id).await?),
            GetRoutineQuery::ByName(name) => {
                let routine_name = RoutineName::new(name)?;
                Ok(self.repo.get_by_name(&routine_name).await?)
            }
        }
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

    pub async fn add_set(&self, cmd: &AddSetCommand) -> Result<Routine, AddSetError> {
        let mut routine: Routine = self
            .repo
            .get_by_id(cmd.routine_id)
            .await?
            .ok_or(AddSetError::RoutineNotFound(cmd.routine_id()))?;

        let new_set: Set = Set::new(cmd.new_set_id(), cmd.number_of_reps.unwrap_or(10u8));
        routine.add_set_to_exercise(cmd.exercise_id(), new_set)?;

        self.repo.save(&routine).await?;

        Ok(routine)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateRoutineCommand {
    routine_id: Uuid,
    name: String,
}

impl CreateRoutineCommand {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            routine_id: Uuid::now_v7(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }
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
    routine_id: Uuid,
    new_name: String,
}

impl RenameRoutineCommand {
    pub fn new(routine_id: Uuid, new_name: impl Into<String>) -> Self {
        Self {
            routine_id,
            new_name: new_name.into(),
        }
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }
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

#[derive(Clone, Debug)]
pub struct AddSetCommand {
    routine_id: Uuid,
    exercise_id: Uuid,
    new_set_id: Uuid,
    number_of_reps: Option<u8>,
}

impl AddSetCommand {
    pub fn new(routine_id: Uuid, exercise_id: Uuid) -> Self {
        Self {
            routine_id,
            exercise_id,
            new_set_id: Uuid::now_v7(),
            number_of_reps: None,
        }
    }

    pub fn with_reps(mut self, reps: u8) -> Self {
        self.number_of_reps = Some(reps);
        self
    }

    pub fn exercise_id(&self) -> Uuid {
        self.exercise_id
    }

    pub fn new_set_id(&self) -> Uuid {
        self.new_set_id
    }

    pub fn number_of_reps(&self) -> Option<u8> {
        self.number_of_reps
    }

    pub fn routine_id(&self) -> Uuid {
        self.routine_id
    }
}

#[derive(Debug, Error)]
pub enum AddSetError {
    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("domain rule violation: {0}")]
    Domain(#[from] RoutineError),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetRoutineQuery {
    ById(Uuid),
    ByName(String),
}

#[derive(Debug, Error)]
pub enum GetRoutineError {
    #[error(transparent)]
    Validation(#[from] RoutineNameEmptyError),

    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("domain rule violation: {0}")]
    Domain(#[from] RoutineError),

    #[error("repository error: {0}")]
    Repository(#[from] RoutineRepositoryError),
}
