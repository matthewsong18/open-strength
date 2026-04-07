use thiserror::Error;
use uuid::Uuid;

use super::ports::{WorkoutRepository, WorkoutRepositoryError};
use super::workout::Workout;
use crate::routine::ports::{RoutineRepository, RoutineRepositoryError};
use crate::shared::{intensity::Intensity, weight::Weight};

#[derive(Debug, Clone)]
pub struct WorkoutService<WR: WorkoutRepository, RR: RoutineRepository> {
    workout_repo: WR,
    routine_repo: RR,
}

impl<WR, RR> WorkoutService<WR, RR>
where
    WR: WorkoutRepository,
    RR: RoutineRepository,
{
    pub fn new(workout_repo: WR, routine_repo: RR) -> Self {
        Self {
            workout_repo,
            routine_repo,
        }
    }

    pub async fn start_workout_from_routine(
        &self,
        routine_id: Uuid,
    ) -> Result<Workout, WorkoutServiceError> {
        let routine = self
            .routine_repo
            .get_by_id(routine_id)
            .await?
            .ok_or(WorkoutServiceError::RoutineNotFound(routine_id))?;

        let workout = Workout::from_routine(&routine);
        self.workout_repo.save(&workout).await?;

        Ok(workout)
    }

    pub async fn start_freestyle_workout(&self) -> Result<Workout, WorkoutServiceError> {
        let workout = Workout::new_freestyle();
        self.workout_repo.save(&workout).await?;
        Ok(workout)
    }

    pub async fn log_set(
        &self,
        cmd: &LogSetPerformanceCommand,
    ) -> Result<Workout, WorkoutServiceError> {
        let mut workout = self
            .workout_repo
            .get_by_id(cmd.workout_id)
            .await?
            .ok_or(WorkoutServiceError::WorkoutNotFound(cmd.workout_id))?;

        workout
            .log_set_performance(
                cmd.exercise_id,
                cmd.set_id,
                cmd.reps,
                cmd.weight,
                cmd.intensity,
            )
            .map_err(WorkoutServiceError::Domain)?;

        self.workout_repo.save(&workout).await?;
        Ok(workout)
    }

    pub async fn finish_workout(&self, workout_id: Uuid) -> Result<Workout, WorkoutServiceError> {
        let mut workout = self
            .workout_repo
            .get_by_id(workout_id)
            .await?
            .ok_or(WorkoutServiceError::WorkoutNotFound(workout_id))?;

        workout.set_completed();
        self.workout_repo.save(&workout).await?;

        Ok(workout)
    }
}

pub struct LogSetPerformanceCommand {
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
    pub set_id: Uuid,
    pub reps: u8,
    pub weight: Option<Weight>,
    pub intensity: Option<Intensity>,
}

#[derive(Debug, Error)]
pub enum WorkoutServiceError {
    #[error("routine with id {0} could not be found")]
    RoutineNotFound(Uuid),

    #[error("workout with id {0} could not be found")]
    WorkoutNotFound(Uuid),

    #[error("repository error: {0}")]
    RoutineRepository(#[from] RoutineRepositoryError),

    #[error("repository error: {0}")]
    WorkoutRepository(#[from] WorkoutRepositoryError),

    #[error("domain error: {0}")]
    Domain(String),
}
