use std::sync::Arc;

use dioxus::prelude::*;
use domain::{common::Exercise, routine::Routine, routine_repository::RoutineRepository};

use crate::Route;

#[component]
pub fn NewRoutine() -> Element {
    let repo = use_context::<Arc<dyn RoutineRepository>>();
    let navigator = use_navigator();

    let mut new_routine = use_signal(Routine::new);
    let mut new_exercise = use_signal(Exercise::default);

    let mut is_adding_exercise = use_signal(|| false);

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

    let add_new_exercise = move |event: Event<FormData>| {
        event.prevent_default();

        let exercise_to_add = new_exercise.read().clone();

        new_routine.write().add_exercise(exercise_to_add);

        new_exercise.set(Exercise::default());
        is_adding_exercise.set(false);
    };

    if *is_adding_exercise.read() {
        return rsx! {
            main {
                h2 { "Add an Exercise" }

                form {
                    onsubmit: add_new_exercise,

                    div {
                        label { "Exercise Name:" }
                        input {
                            type: "text",
                            value: "{new_exercise.read().name()}",
                            oninput: move |event| new_exercise.write().set_name(event.value()),
                            required: true
                        }
                    }

                    div {
                        label { "Equipment:" }
                        input {
                            type: "text",
                            value: "{new_exercise.read().equipment()}",
                            oninput: move |event| new_exercise.write().set_equipment(event.value()),
                            required: true
                        }
                    }

                    for (index, set) in new_exercise.read().get_sets().iter().enumerate() {
                        div {
                            label { "Reps:" }
                            input {
                                type: "number",
                                value: "{set.reps()}",
                                oninput: move |event| {
                                    if let Ok(reps) = event.value().parse::<u8>() {
                                        let _ = new_exercise.write().update_set_reps(index, reps);
                                    }
                                }
                            }
                        }
                    }

                    button {
                        type: "button",
                        onclick: move |_| new_exercise.write().add_set(10),
                        "Add A Set"
                    }

                    button {
                        type: "submit",
                        "Add Exercise"
                    }

                    button {
                        type: "button",
                        onclick: move |_| is_adding_exercise.set(false),
                        "Cancel"
                    }
                }
            }

        };
    }

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

                for exercise in new_routine.read().get_exercises() {
                    div {
                        p { "{exercise.name()}" }
                        p { "Sets: {exercise.get_sets().len()}" }
                    }
                }

                button {
                    type: "button",
                    onclick: move |_| is_adding_exercise.set(true),
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
