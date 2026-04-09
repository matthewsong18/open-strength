use dioxus::prelude::*;
use domain::routine::{
    memory_routine_repository::MemoryRoutineRepository, service::RoutineService,
};

use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let service = use_context::<RoutineService<MemoryRoutineRepository>>();

    let routines = use_resource(move || {
        let service_clone = service.clone();

        async move { service_clone.get_all_routines().await.unwrap_or_default() }
    });

    let navigator = use_navigator();

    rsx! {
        main {
            h1 { "Your Routine Library" }

            section {
                if let Some(routine_list) = routines.read().as_ref() {

                    for (r, id) in routine_list.iter().map(|r| (r, r.id())) {
                        article {
                            h2 { "{r.name()}" }

                            p {
                                "Number of Exercises: {r.get_exercises().len()}"
                            }

                            button {
                                onclick: move |_| {
                                    navigator.push(Route::ViewRoutine { id });
                                },
                                "View Routine"
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
