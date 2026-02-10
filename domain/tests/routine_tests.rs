mod common;
use domain::{common::Exercise, routine::Routine};

use common::create_test_exercise;

#[test]
fn test_init_workout() {
    let routine = Routine::new();
    assert_eq!(routine.exercise_count(), 0);
}

#[test]
fn test_add_exercise() {
    let mut routine: Routine = Routine::new();
    create_test_exercise(&mut routine);
    assert_eq!(routine.exercise_count(), 1);
    let exercise: &Exercise = routine.get_exercise(0).unwrap();
    assert_eq!(exercise.get_sets().len(), 3);
}

#[test]
fn test_update_the_target_reps_of_a_set() {
    let mut routine = Routine::new();
    create_test_exercise(&mut routine);

    routine.update_set_target_reps(0, 0, 7).unwrap();

    let exercise: &Exercise = routine.get_exercise(0).unwrap();
    assert_eq!(exercise.get_sets()[0].get_reps(), 7);
}
