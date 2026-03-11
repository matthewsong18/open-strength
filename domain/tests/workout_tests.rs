use domain::{common::Exercise, routine::Routine, workout::Workout};

#[test]
fn test_start_live_workout() {
    let mut routine: Routine = Routine::default().with_name("Push Day");
    routine.add_exercise(Exercise::default().with_sets(3, 10));
    routine.add_exercise(Exercise::default().with_sets(3, 10));
    routine.add_exercise(Exercise::default().with_sets(3, 10));

    let workout: Workout = Workout::from_routine(&routine);

    let exercises = workout.get_exercises();
    assert_eq!(3, exercises.len());

    workout.get_exercises().iter().for_each(|exercise| {
        let set_count = exercise.get_sets().iter().count();
        assert_eq!(3, set_count);

        exercise
            .get_sets()
            .iter()
            .for_each(|set| assert_eq!(10, set.reps()));
    });
}
