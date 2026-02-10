use domain::common::Exercise;
use domain::routine::Routine;

pub fn create_test_exercise(routine: &mut Routine) {
    let exercise = Exercise::new("Test".to_string(), "Test".to_string())
        .with_sets(3, 10);
    routine.add_exercise(exercise);
}
