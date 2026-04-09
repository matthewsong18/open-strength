use dioxus::prelude::*;
use uuid::Uuid;

// struct RoutineView {
//     id: String,
//     name: String,
//     created_at: String,
//     exercises: Vec<ExerciseView>,
// }

// struct ExerciseView {
//     id: String,
//     name: String,
//     equipment: String,
//     sets: Vec<SetView>,
// }

// struct SetView {
//     id: String,
//     reps: u8,
//     weight: u32,
//     intensity: u8,
// }

#[component]
pub fn ViewRoutine(id: Uuid) -> Element {
    rsx! {
        div {
            h1 { "Viewing Routine" }
            p { "Routine ID: {id}" }
        }
    }
}
