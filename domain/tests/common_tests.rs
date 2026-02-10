use domain::common::Exercise;

#[test]
fn test_exercise_add_set() {
    let mut exercise = Exercise::new("Chest Press".to_string(), "Bench Press".to_string());

    exercise.add_set().unwrap();
}
