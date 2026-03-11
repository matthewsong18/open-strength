use domain::common::Exercise;
use domain::routine::Routine;
use domain::workout::Workout;

pub fn create_test_exercise(routine: &mut Routine) {
    let exercise = Exercise::new("Test".to_string(), "Test".to_string()).with_sets(3, 10);
    routine.add_exercise(exercise);
}

pub fn create_test_workout() -> Workout {
    let mut routine: Routine = Routine::default().with_name("Push Day");
    routine.add_exercise(Exercise::default().with_sets(3, 10));
    routine.add_exercise(Exercise::default().with_sets(3, 10));
    routine.add_exercise(Exercise::default().with_sets(3, 10));

    Workout::from_routine(&routine)
}
