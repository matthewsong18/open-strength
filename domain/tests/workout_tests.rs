use domain::routine::{
    memory_routine_repository::MemoryRoutineRepository,
    service::{AddExerciseToRoutineCommand, CreateRoutineCommand, RoutineService},
};
use domain::workout::{
    memory_workout_repository::MemoryWorkoutRepository,
    service::{WorkoutService},
};

#[tokio::test]
async fn test_start_workout_from_routine() -> anyhow::Result<()> {
    let routine_repo = MemoryRoutineRepository::new();
    let workout_repo = MemoryWorkoutRepository::new();
    
    let routine_service = RoutineService::new(routine_repo.clone());
    let workout_service = WorkoutService::new(workout_repo.clone(), routine_repo.clone());

    // 1. Create a routine
    let routine_name = "Push Day";
    let create_cmd = CreateRoutineCommand::new(routine_name);
    let routine = routine_service.create_routine(&create_cmd).await?;

    // 2. Add an exercise to it
    let add_exercise_cmd = AddExerciseToRoutineCommand::new(routine.id(), "Bench Press")
        .with_sets_and_reps(3, 10);
    routine_service.add_exercise(&add_exercise_cmd).await?;

    // 3. Start a workout from that routine
    let workout = workout_service.start_workout_from_routine(routine.id()).await?;

    assert_eq!(workout.name(), routine_name);
    assert_eq!(workout.get_exercises().len(), 1);
    assert_eq!(workout.get_exercises()[0].sets().len(), 3);
    assert!(workout.completed_at().is_none());

    Ok(())
}

#[tokio::test]
async fn test_finish_workout() -> anyhow::Result<()> {
    let routine_repo = MemoryRoutineRepository::new();
    let workout_repo = MemoryWorkoutRepository::new();
    let workout_service = WorkoutService::new(workout_repo.clone(), routine_repo.clone());

    let workout = workout_service.start_freestyle_workout().await?;
    assert!(workout.completed_at().is_none());

    let finished_workout = workout_service.finish_workout(workout.id()).await?;
    assert!(finished_workout.completed_at().is_some());

    Ok(())
}
