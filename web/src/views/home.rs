use dioxus::prelude::*;
use domain::routine::Routine;

use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let routines = use_signal(|| {
        vec![
            Routine::new().with_name("Push Day"),
            Routine::new().with_name("Pull Day"),
            Routine::new().with_name("Leg Day"),
        ]
    });

    rsx! {
        main {
            h1 { "Your Routine Library" }

            section {
                for r in routines.read().iter() {
                    article {
                        h2 { "{r.name()}" }
                        button {
                            "Start Workout"
                        }
                    }
                }
            }

            Link {
                to: Route::NewRoutine{},
                "New Routine"
            }
        }
    }
}
