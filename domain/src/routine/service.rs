use crate::routine::models::exercise::{EquipmentName, ExerciseName};

use super::{
    models::{
        exercise::Exercise,
        root::{
            AddExerciseToRoutineCommand, AddExerciseToRoutineError, CreateRoutineCommand,
            CreateRoutineError, RenameRoutineCommand, RenameRoutineError, Routine, RoutineName,
        },
    },
    ports::{RoutineRepository, RoutineService},
};

#[derive(Debug, Clone)]
pub struct Service<R: RoutineRepository> {
    repo: R,
}

impl<R> Service<R>
where
    R: RoutineRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> RoutineService for Service<R>
where
    R: RoutineRepository,
{
    async fn create_routine(
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

    async fn rename_routine(
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

    async fn add_exercise(
        &self,
        cmd: &AddExerciseToRoutineCommand,
    ) -> Result<Routine, AddExerciseToRoutineError> {
        let mut routine = self
            .repo
            .get_by_id(cmd.target_id)
            .await?
            .ok_or(AddExerciseToRoutineError::NotFound(cmd.target_id))?;

        let exercise_name: ExerciseName = ExerciseName::new("Chest Press")?;
        let equipment_name: EquipmentName = EquipmentName::new("Bench Press")?;

        let sets = cmd.number_of_sets.unwrap_or(3u8);
        let reps = cmd.number_of_reps.unwrap_or(10u8);

        let exercise = Exercise::new(exercise_name, Some(equipment_name)).with_sets(sets, reps);

        routine.add_exercise(exercise);

        Ok(routine)
    }
}
