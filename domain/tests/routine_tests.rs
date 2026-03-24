use domain::routine::{
    memory_routine_repository::MemoryRoutineRepository,
    models::root::{CreateRoutineRequest, RoutineName},
    ports::RoutineService,
    service::Service,
};

#[tokio::test]
async fn test_create_empty_routine() {
    let repo: MemoryRoutineRepository = MemoryRoutineRepository::new();
    let service: Service<MemoryRoutineRepository> = Service::new(repo);

    let routine_name: RoutineName = RoutineName::new("Push Day").unwrap();
    let request: CreateRoutineRequest = CreateRoutineRequest::new(routine_name);

    let _ = service.create_routine(&request).await.unwrap();
}

// mod common;
// use domain::{common::Exercise, routine::Routine};

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
