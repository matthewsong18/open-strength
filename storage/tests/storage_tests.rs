use domain::routine_repository::RoutineRepository;
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
            }
        )*
    };
}

generate_storage_tests! {
    memory: MemoryRoutineRepository::new()
}
