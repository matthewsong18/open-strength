use domain::common::Exercise;

#[test]
fn test_exercise_add_set() {
    let mut exercise = Exercise::new("Chest Press".to_string(), "Bench Press".to_string());

    let start_count = exercise.get_sets().len();
    assert_eq!(0, start_count);

    exercise.add_set();

    let end_count = exercise.get_sets().len();
    assert_eq!(1, end_count);
}
