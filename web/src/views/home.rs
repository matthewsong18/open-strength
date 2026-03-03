use std::sync::Arc;

use dioxus::prelude::*;
use domain::routine_repository::RoutineRepository;

use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let routine_repo = use_context::<Arc<dyn RoutineRepository>>();

    let routines = use_resource(move || {
        let repo_clone = routine_repo.clone();

        async move { repo_clone.get_all().await.unwrap_or_default() }
    });

    rsx! {
        main {
            h1 { "Your Routine Library" }

            section {
                if let Some(routine_list) = routines.read().as_ref() {

                    for r in routine_list {
                        article {
                            h2 { "{r.name()}" }
                            button {
                                "Start Workout"
                            }
                        }
                    }

                } else {
                    p { "Loading routines..." }
                }
            }

            Link {
                to: Route::NewRoutine{},
                "New Routine"
            }
        }
    }
}
