use std::fmt::{Display, Formatter};

use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intensity(u8);

impl Intensity {
    /// Creates an Intensity from Rate of Perceived Exertion (RPE).
    pub fn from_rpe(rpe: u8) -> Self {
        Intensity(rpe)
    }

    /// Creates an Intensity from Reps in Reserve (RIR).
    /// Assumes a standard 10-point scale (RPE = 10 - RIR).
    pub fn from_rir(rir: u8) -> Self {
        // saturating_sub prevents integer underflow panics if rir > 10
        Intensity(10u8.saturating_sub(rir))
    }

    /// Returns the intensity formatted as RPE.
    pub fn as_rpe(&self) -> u8 {
        self.0
    }

    /// Returns the intensity formatted as RIR.
    pub fn as_rir(&self) -> u8 {
        10u8.saturating_sub(self.0)
    }
}
