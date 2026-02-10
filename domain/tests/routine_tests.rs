use domain::{common::Exercise, routine::Routine};

#[test]
fn init_workout() {
    let workout = Routine::new();
    assert_eq!(workout.exercise_count(), 0);
}

#[test]
fn add_exercise() {
    let mut workout = Routine::new();
    let result = workout.add_exercise("Chest Press".to_string(), "Bench Press".to_string(), 3, 10);
    assert_eq!(Ok(()), result);
    assert_eq!(workout.exercise_count(), 1);
    let exercise: &Exercise = workout.get_exercise(0).unwrap();
    assert_eq!(exercise.get_sets().len(), 3);
}

#[test]
fn update_the_target_reps_of_a_set() {
    let mut workout = Routine::new();
    workout
        .add_exercise("Chest Press".to_string(), "Bench press".to_string(), 3, 10)
        .unwrap();

    workout.update_set_target_reps(0, 0, 7).unwrap();

    let exercise: &Exercise = workout.get_exercise(0).unwrap();
    assert_eq!(exercise.get_sets()[0].get_reps(), 7);
}
