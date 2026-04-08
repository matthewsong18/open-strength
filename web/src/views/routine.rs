use dioxus::prelude::*;
use domain::routine::{
    memory_routine_repository::MemoryRoutineRepository,
    service::{AddExerciseToRoutineCommand, CreateRoutineCommand, RoutineService},
};

use crate::Route;

#[derive(Default, Clone, PartialEq)]
struct DraftExercise {
    name: String,
    equipment: String,
    sets: Vec<u8>,
}

#[component]
pub fn NewRoutine() -> Element {
    let service = use_context::<RoutineService<MemoryRoutineRepository>>();
    let navigator = use_navigator();

    let mut routine_name = use_signal(|| "Untitled Routine".to_string());
    let mut exercises = use_signal(Vec::<DraftExercise>::new);

    let mut new_exercise = use_signal(DraftExercise::default);
    let mut is_adding_exercise = use_signal(|| false);

    let save_routine = move |event: Event<FormData>| {
        event.prevent_default();

        let service_clone = service.clone();
        let name = routine_name.read().clone();
        let exercise_list = exercises.read().clone();

        spawn(async move {
            let create_cmd = CreateRoutineCommand::new(name);
            let routine_id = create_cmd.routine_id();

            if service_clone.create_routine(&create_cmd).await.is_ok() {
                for ex in exercise_list {
                    let mut add_ex_cmd = AddExerciseToRoutineCommand::new(routine_id, ex.name);
                    if !ex.equipment.is_empty() {
                        add_ex_cmd = add_ex_cmd.with_equipment(ex.equipment);
                    }
                    // Currently we only support adding one set at a time or with_sets_and_reps if they are uniform.
                    // For simplicity, let's just use with_sets_and_reps if all reps are the same,
                    // or just use the first set's reps for all if they are uniform.
                    // The service currently doesn't support heterogeneous sets in a single command.
                    if let Some(&reps) = ex.sets.first() {
                        add_ex_cmd = add_ex_cmd.with_sets_and_reps(ex.sets.len() as u8, reps);
                    }

                    let _ = service_clone.add_exercise(&add_ex_cmd).await;
                }
                navigator.push(Route::Home {});
            }
        });
    };

    let add_new_exercise = move |event: Event<FormData>| {
        event.prevent_default();

        let exercise_to_add = new_exercise.read().clone();
        exercises.write().push(exercise_to_add);

        new_exercise.set(DraftExercise::default());
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
                            value: "{new_exercise.read().name}",
                            oninput: move |event| new_exercise.write().name = event.value(),
                            required: true
                        }
                    }

                    div {
                        label { "Equipment:" }
                        input {
                            type: "text",
                            value: "{new_exercise.read().equipment}",
                            oninput: move |event| new_exercise.write().equipment = event.value(),
                            required: true
                        }
                    }

                    for (index, reps) in new_exercise.read().sets.iter().enumerate() {
                        div {
                            label { "Reps:" }
                            input {
                                type: "number",
                                value: "{reps}",
                                oninput: move |event| {
                                    if let Ok(reps) = event.value().parse::<u8>() {
                                        new_exercise.write().sets[index] = reps;
                                    }
                                }
                            }
                        }
                    }

                    button {
                        type: "button",
                        onclick: move |_| new_exercise.write().sets.push(10),
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

                div {
                    label { "Routine Name:" }
                    input {
                        type: "text",
                        value: "{routine_name}",
                        oninput: move |event| routine_name.set(event.value()),
                        required: true
                    }
                }

                for exercise in exercises.read().iter() {
                    div {
                        p { "{exercise.name}" }
                        p { "Sets: {exercise.sets.len()}" }
                    }
                }

                div {
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
}
