use crate::shared::{intensity::Intensity, weight::Weight};

use uuid::Uuid;

/// These are the sets that make up each exercise and the main tracking target
/// for progress.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Set {
    id: Uuid,
    reps: u8,
    weight: Option<Weight>,
    intensity: Option<Intensity>,
}

impl Set {
    pub fn new(reps: u8) -> Self {
        Self {
            id: Uuid::now_v7(),
            reps,
            weight: None,
            intensity: None,
        }
    }

    // Getters

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn weight(&self) -> &Option<Weight> {
        &self.weight
    }

    pub fn intensity(&self) -> &Option<Intensity> {
        &self.intensity
    }

    pub fn reps(&self) -> &u8 {
        &self.reps
    }

    // Setters

    pub fn set_reps(&mut self, new_reps: u8) {
        self.reps = new_reps
    }
}
