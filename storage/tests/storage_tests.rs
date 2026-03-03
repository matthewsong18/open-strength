use domain::{routine::Routine, routine_repository::RoutineRepository};
use storage::memory_routine_repository::MemoryRoutineRepository;

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

                    let new_routine = Routine::new();
                    repo.save(new_routine).await.unwrap();
                    assert_eq!(1, repo.get_all().await.unwrap().len());
                }

                #[tokio::test]
                async fn test_get_by_id() {
                    let repo = $setup;

                    let new_routine = Routine::new();
                    let target_id = new_routine.id();
                    repo.save(new_routine).await.unwrap();

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
