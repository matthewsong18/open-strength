use domain::routine::{
    memory_routine_repository::MemoryRoutineRepository,
    models::root::{Routine, RoutineName},
    ports::RoutineRepository,
};

macro_rules! generate_storage_tests {
    ($($mod_name:ident: $setup: expr),*) => {
        $(
            mod $mod_name {
                use super::*;

                #[tokio::test]
                async fn test_default() {
                    let repo = $setup;
                    assert_eq!(0, repo.get_all().await.unwrap().len());
                }

                #[tokio::test]
                async fn test_save_routine() {
                    let repo = $setup;
                    assert_eq!(0, repo.get_all().await.unwrap().len());

                    let name = RoutineName::new("Push Day").expect("name shouldn't be invalid");

                    let routine_id = uuid::Uuid::now_v7();
                    let routine: Routine = Routine::new(routine_id, name);
                    repo.save(&routine).await.unwrap();
                    assert_eq!(1, repo.get_all().await.unwrap().len());
                }

                #[tokio::test]
                async fn test_get_by_id() {
                    let repo = $setup;

                    let name = RoutineName::new("Push Day").expect("name shouldn't be invalid");
                    let routine_id = uuid::Uuid::now_v7();
                    let routine: Routine = Routine::new(routine_id, name);

                    repo.save(&routine).await.unwrap();

                    let target_id = routine.id();
                    let result_routine = repo.get_by_id(target_id).await.unwrap().unwrap();
                    assert_eq!(target_id, result_routine.id());
                }
            }
        )*
    };
}

generate_storage_tests! {
    memory: MemoryRoutineRepository::new()
}
