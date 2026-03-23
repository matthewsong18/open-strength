// mod common;
// use common::create_test_workout;

// #[test]
// fn test_start_live_workout() {
//     let workout = create_test_workout();

//     let exercises = workout.get_exercises();
//     assert_eq!(3, exercises.len());

//     workout.get_exercises().iter().for_each(|exercise| {
//         let set_count = exercise.get_sets().iter().count();
//         assert_eq!(3, set_count);

//         exercise
//             .get_sets()
//             .iter()
//             .for_each(|set| assert_eq!(10, set.reps()));
//     });
// }
