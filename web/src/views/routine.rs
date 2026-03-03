use std::sync::Arc;

use dioxus::prelude::*;
use domain::{common::Exercise, routine::Routine, routine_repository::RoutineRepository};

use crate::Route;

#[component]
pub fn NewRoutine() -> Element {
    let repo = use_context::<Arc<dyn RoutineRepository>>();
    let navigator = use_navigator();

    let mut new_routine = use_signal(Routine::new);

    let save_routine = move |event: Event<FormData>| {
        event.prevent_default();

        let repo_clone = repo.clone();

        let routine_to_save = new_routine.read().clone();

        spawn(async move {
            match repo_clone.save(routine_to_save).await {
                Ok(_) => navigator.push(Route::Home {}),
                Err(_) => todo!(),
            };
        });
    };

    let add_new_exercise = move |_| {
        let new_exercise = Exercise::new(
            "Exercise Name".to_string(),
            "Exercise Equipment".to_string(),
        )
        .with_sets(3, 10);

        new_routine.write().add_exercise(new_exercise);
    };

    rsx! {
        main {
            h1 { "Create a New Routine" }

            form {
                onsubmit: save_routine,

                input {
                    type: "text",
                    value: "{new_routine.read().name()}",
                    oninput: move |event| new_routine.write().set_name(event.value()),
                    required: true
                }

                button {
                    type: "button",
                    onclick: add_new_exercise,
                    "Add Exercise"
                }

                button {
                    type: "submit",
                    "Save Routine"
                }
            }
        }
    }
}
