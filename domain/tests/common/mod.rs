use domain::routine::Routine;

pub fn create_test_exercise(routine: &mut Routine) -> Result<(), String> {
    routine.add_exercise("Test".to_string(), "Test".to_string(), 3, 10)
}
