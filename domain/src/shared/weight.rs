use std::fmt::{Display, Formatter};

use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weight(u32);

impl Weight {
    pub fn from_kg(kg: f32) -> Self {
        Weight((kg * 1000.0).round() as u32)
    }

    pub fn from_lbs(lbs: f32) -> Self {
        Weight((lbs * 453.592).round() as u32)
    }

    /// Returns the rounded kg weight as a whole integer or `.5`
    pub fn as_kg(&self) -> f32 {
        let exact_kg = self.0 as f32 / 1000.0;
        (exact_kg * 2.0).round() / 2.0
    }

    /// Returns the rounded lbs weight as a whole integer
    pub fn as_lbs(&self) -> f32 {
        let exact_lbs = self.0 as f32 / 453.592;
        exact_lbs.round()
    }
}
