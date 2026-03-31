use domain::routine::{
    memory_routine_repository::MemoryRoutineRepository,
    service::{
        AddExerciseToRoutineCommand, AddSetCommand, CreateRoutineCommand, CreateRoutineError,
        GetRoutineQuery, RenameExerciseCommand, RenameRoutineCommand, RenameRoutineError,
        RoutineService,
    },
};

fn get_test_service() -> RoutineService<MemoryRoutineRepository> {
    get_test_service_with_repo(MemoryRoutineRepository::new())
}

fn get_test_service_with_repo(
    repo: MemoryRoutineRepository,
) -> RoutineService<MemoryRoutineRepository> {
    RoutineService::new(repo)
}

#[tokio::test]
async fn test_create_empty_routine() -> anyhow::Result<()> {
    let service = get_test_service();
    let routine_name = "Push Day".to_string();
    let request: CreateRoutineCommand = CreateRoutineCommand::new(routine_name.clone());
    let test_routine = service.create_routine(&request).await?;

    assert_eq!(routine_name, *test_routine.name().to_string());

    Ok(())
}

#[tokio::test]
async fn test_rename_routine() -> anyhow::Result<()> {
    let service = get_test_service();

    let routine_name = "Chest Day".to_string();
    let test_routine_request = CreateRoutineCommand::new(routine_name.clone());
    let test_routine = service.create_routine(&test_routine_request).await?;

    let new_routine_name = "Push Day".to_string();
    let request = RenameRoutineCommand::new(test_routine.id(), &new_routine_name);
    let updated_routine = service.rename_routine(&request).await?;

    assert_eq!(new_routine_name, updated_routine.name().to_string());

    Ok(())
}

#[tokio::test]
async fn test_add_valid_exercise() -> anyhow::Result<()> {
    let repo = MemoryRoutineRepository::new();
    let service = get_test_service_with_repo(repo.clone());

    let routine_name = "Chest Day";

    let create_routine_command = CreateRoutineCommand::new(routine_name);
    let routine = service.create_routine(&create_routine_command).await?;

    let routine_id = routine.id();
    let add_exercise_command = AddExerciseToRoutineCommand::new(routine_id, "Chest Press");
    let exercise_id = add_exercise_command.new_exercise_id();
    service.add_exercise(&add_exercise_command).await?;

    let new_service = get_test_service_with_repo(repo.clone());
    let test_routine = new_service
        .get_routine(&GetRoutineQuery::ById(routine_id))
        .await?
        .unwrap();

    let result = test_routine
        .get_exercises()
        .iter()
        .find(|e| e.id() == exercise_id);

    assert!(result.is_some());

    Ok(())
}

#[tokio::test]
async fn test_create_duplicate_routine_returns_error() -> anyhow::Result<()> {
    let service = get_test_service();

    let request = CreateRoutineCommand::new("Original");
    // create the initial routine successfully
    service.create_routine(&request).await?;

    // attempt to create the duplicate and extract the error
    let err = service.create_routine(&request).await.unwrap_err();

    assert!(
        matches!(err, CreateRoutineError::Duplicate { .. }),
        "Expected Duplicate error, but got: {:?}",
        err
    );

    Ok(())
}

#[tokio::test]
async fn test_rename_to_duplicate_routine_returns_error() -> anyhow::Result<()> {
    let service = get_test_service();

    // setup
    let original_request = CreateRoutineCommand::new("Original");
    service.create_routine(&original_request).await?;

    let new_request = CreateRoutineCommand::new("New");
    let new_routine = service.create_routine(&new_request).await?;

    // attempt to rename the second routine to the first routine's name
    let rename_request = RenameRoutineCommand::new(new_routine.id(), "Original");
    let err = service.rename_routine(&rename_request).await.unwrap_err();

    assert!(
        matches!(err, RenameRoutineError::Duplicate { .. }),
        "Expected Duplicate error, but got: {:?}",
        err
    );

    Ok(())
}

#[tokio::test]
async fn test_rename_exercise_succeeds() -> anyhow::Result<()> {
    let repo = MemoryRoutineRepository::new();
    let service = get_test_service_with_repo(repo.clone());

    // setup

    let new_routine_cmd = CreateRoutineCommand::new("Routine");
    let routine = service.create_routine(&new_routine_cmd).await?;

    let new_exercise_cmd = AddExerciseToRoutineCommand::new(routine.id(), "Exercise1");
    let updated_routine = service.add_exercise(&new_exercise_cmd).await?;

    // attempt to rename exercise

    let exercise_id = new_exercise_cmd.new_exercise_id();
    let new_name = "RenamedExercise".to_string();

    let result_routine_id = service
        .rename_exercise(&RenameExerciseCommand::new(
            updated_routine.id(),
            exercise_id,
            new_name.clone(),
        ))
        .await?
        .id();

    // assert that routine rename persisted with a new service

    let new_service = get_test_service_with_repo(repo.clone());
    let test_routine = new_service
        .get_routine(&GetRoutineQuery::ById(result_routine_id))
        .await?
        .unwrap();

    let updated_exercise_name = test_routine
        .get_exercise(exercise_id)
        .unwrap()
        .name()
        .to_string();

    assert_eq!(new_name, updated_exercise_name);

    Ok(())
}

#[tokio::test]
async fn test_add_set_success() -> anyhow::Result<()> {
    let repo = MemoryRoutineRepository::new();
    let service = get_test_service_with_repo(repo.clone());

    let routine_id = service
        .create_routine(&CreateRoutineCommand::new("Routine"))
        .await?
        .id();

    let add_exercise_cmd = AddExerciseToRoutineCommand::new(routine_id, "Bench Press");
    service.add_exercise(&add_exercise_cmd).await?;

    let add_set_cmd = AddSetCommand::new(routine_id, add_exercise_cmd.new_exercise_id());
    service.add_set(&add_set_cmd).await?;

    // assert that a set exists that matches the expected set id

    let new_service = get_test_service_with_repo(repo.clone());
    let test_routine = new_service
        .get_routine(&GetRoutineQuery::ById(routine_id))
        .await?
        .unwrap();

    let exercise_id = add_exercise_cmd.new_exercise_id();
    let expected_set_id = add_set_cmd.new_set_id();
    test_routine
        .get_exercise(exercise_id)
        .unwrap()
        .sets()
        .iter()
        .rfind(|s| s.id() == expected_set_id)
        .unwrap();

    Ok(())
}

#[tokio::test]
async fn test_get_routine() -> anyhow::Result<()> {
    let repo = MemoryRoutineRepository::new();
    let service = get_test_service_with_repo(repo.clone());

    let expected_routine_id = service
        .create_routine(&CreateRoutineCommand::new("Routine"))
        .await?
        .id();

    let new_service = get_test_service_with_repo(repo);

    let actual_id_from_id = new_service
        .get_routine(&GetRoutineQuery::ById(expected_routine_id))
        .await?
        .unwrap()
        .id();
    let actual_id_from_name = new_service
        .get_routine(&GetRoutineQuery::ByName("Routine".to_string()))
        .await?
        .unwrap()
        .id();

    assert_eq!(expected_routine_id, actual_id_from_id);
    assert_eq!(expected_routine_id, actual_id_from_name);
    Ok(())
}

// #[test]
// fn test_init_routine() {
//     let routine = Routine::new();
//     assert_eq!(routine.exercise_count(), 0);
// }

// #[test]
// fn test_add_exercise() {
//     let mut routine: Routine = Routine::new();
//     create_test_exercise(&mut routine);
//     assert_eq!(routine.exercise_count(), 1);
//     let exercise: &Exercise = routine.get_exercise(0).unwrap();
//     assert_eq!(exercise.get_sets().len(), 3);
// }

// #[test]
// fn test_update_the_target_reps_of_a_set() {
//     let mut routine = Routine::new();
//     create_test_exercise(&mut routine);

//     routine.update_set_target_reps(0, 0, 7).unwrap();

//     let exercise: &Exercise = routine.get_exercise(0).unwrap();
//     assert_eq!(exercise.get_sets()[0].reps(), 7);

//     // Testing wrong index
//     routine.update_set_target_reps(10, 0, 10).unwrap_err();
//     routine.update_set_target_reps(0, 10, 10).unwrap_err();
// }

// #[test]
// fn test_adding_sets_to_routine() {
//     let mut routine = Routine::new();
//     let initial_exercise = Exercise::new("Test".to_string(), "Test".to_string());
//     assert_eq!(0, initial_exercise.get_sets().len());

//     routine.add_exercise(initial_exercise);

//     let exercise_id = routine.get_exercise(0).unwrap().id();
//     routine.add_set_to_exercise(exercise_id, 10).unwrap();

//     let modified_exercise = routine.get_exercise(0).unwrap();
//     let new_set_count = modified_exercise.get_sets().len();
//     assert_eq!(1, new_set_count);
// }

// #[test]
// fn test_add_name_to_routine() {
//     let mut routine: Routine = Routine::new();

//     assert_eq!("Untitled Routine", routine.name());

//     routine = routine.with_name("Push Day");

//     assert_eq!("Push Day", routine.name());
// }

// #[test]
// fn test_viewing_exercises() {
//     let mut routine = Routine::new();
//     routine.add_exercise(Exercise::new("Chest Press", "Bench Press").with_sets(3, 10));
//     routine.add_exercise(Exercise::new("Leg Press", "Leg Machine").with_sets(2, 8));

//     let exercises = routine.get_exercises();

//     assert_eq!(2, exercises.len());
//     let chest_exercise = exercises
//         .iter()
//         .find(|e| e.name() == "Chest Press")
//         .expect("chest_exercise shouldn't be None");

//     let chest_sets = 3;
//     let chest_reps = 10;
//     assert_eq!(chest_sets, chest_exercise.get_sets().len());
//     assert_eq!(
//         chest_reps,
//         chest_exercise
//             .get_sets()
//             .first()
//             .expect("should be at least one set")
//             .reps()
//     );

//     let leg_exercise = exercises
//         .iter()
//         .find(|e| e.name() == "Leg Press")
//         .expect("leg_exercise shouldn't be None");

//     let leg_sets = 2;
//     let leg_reps = 8;
//     assert_eq!(leg_sets, leg_exercise.get_sets().len());
//     assert_eq!(
//         leg_reps,
//         leg_exercise
//             .get_sets()
//             .first()
//             .expect("should be at least one set")
//             .reps()
//     );
// }
