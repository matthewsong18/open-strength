use anyhow::anyhow;

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

        let routine: Routine = Routine::new(routine_name);

        match self.repo.save(&routine).await {
            Ok(_) => Ok(routine),
            Err(err) => Err(CreateRoutineError::Unknown(anyhow!(err))),
        }
    }

    async fn rename_routine(
        &self,
        cmd: &RenameRoutineCommand,
    ) -> Result<Routine, RenameRoutineError> {
        let mut routine = self
            .repo
            .get_by_id(cmd.target_id)
            .await
            .map_err(|e| RenameRoutineError::Unknown(anyhow!(e)))?
            .ok_or_else(|| RenameRoutineError::NotFound(cmd.target_id))?;

        let routine_name = RoutineName::try_from(cmd.new_name.clone())?;
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
            .await
            .map_err(|e| AddExerciseToRoutineError::Unknown(anyhow!(e)))?
            .ok_or_else(|| AddExerciseToRoutineError::NotFound(cmd.target_id))?;

        let exercise_name: ExerciseName = ExerciseName::new("Chest Press")
            .map_err(|e| AddExerciseToRoutineError::Unknown(anyhow!(e)))?;
        let equipment_name: EquipmentName = EquipmentName::new("Bench Press")
            .map_err(|e| AddExerciseToRoutineError::Unknown(anyhow!(e)))?;

        let sets = cmd.number_of_sets.unwrap_or(3u8);
        let reps = cmd.number_of_reps.unwrap_or(10u8);

        let exercise = Exercise::new(exercise_name, Some(equipment_name)).with_sets(sets, reps);

        routine.add_exercise(exercise);

        Ok(routine)
    }
}
