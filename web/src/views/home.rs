use dioxus::prelude::*;
use domain::routine::{memory_routine_repository::MemoryRoutineRepository, service::RoutineService};

use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let service = use_context::<RoutineService<MemoryRoutineRepository>>();

    let routines = use_resource(move || {
        let service_clone = service.clone();

        async move {
            service_clone
                .get_all_routines()
                .await
                .unwrap_or_default()
        }
    });

    rsx! {
        main {
            h1 { "Your Routine Library" }

            section {
                if let Some(routine_list) = routines.read().as_ref() {

                    for r in routine_list {
                        article {
                            h2 { "{r.name()}" }

                            p {
                                "Number of Exercises: {r.get_exercises().len()}"
                            }

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
