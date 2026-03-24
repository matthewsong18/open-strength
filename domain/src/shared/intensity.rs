use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intensity(u8);

#[derive(Clone, Debug, Error)]
#[error("intensity ({invalid_value}) must be between 0 - 10")]
pub struct IntensityInvalidRangeError {
    pub invalid_value: u8,
}

impl Intensity {
    /// Creates an Intensity from Rate of Perceived Exertion (RPE).
    pub fn from_rpe(rpe: u8) -> Result<Self, IntensityInvalidRangeError> {
        match (0..=10).contains(&rpe) {
            false => Err(IntensityInvalidRangeError { invalid_value: rpe }),
            true => Ok(Intensity(rpe)),
        }
    }

    /// Creates an Intensity from Reps in Reserve (RIR).
    /// Assumes a standard 10-point scale (RPE = 10 - RIR).
    pub fn from_rir(rir: u8) -> Result<Self, IntensityInvalidRangeError> {
        match (0..=10).contains(&rir) {
            false => Err(IntensityInvalidRangeError { invalid_value: rir }),
            // saturating_sub prevents integer underflow panics if rir > 10
            true => Ok(Intensity(10u8.saturating_sub(rir))),
        }
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

impl Display for Intensity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(RPE: {})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_greater_than_ten_value() {
        let mut failures = Vec::new();

        for value in 11..=u8::MAX {
            if let Ok(e) = Intensity::from_rpe(value) {
                failures.push(format!(
                    "from_rpe incorrectly succeeded for value {}: {}",
                    value, e
                ));
            }

            if let Ok(e) = Intensity::from_rir(value) {
                failures.push(format!(
                    "from_rir incorrectly succeeded for value {}: {}",
                    value, e
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "Test failed with the following errors:\n{}",
                failures.join("\n")
            );
        }
    }

    #[test]
    fn test_valid_range_from_zero_to_ten() {
        let mut failures = Vec::new();

        for value in 0..=10 {
            if let Err(e) = Intensity::from_rpe(value) {
                failures.push(format!("from_rpe failed for value {}: {}", value, e));
            }

            if let Err(e) = Intensity::from_rir(value) {
                failures.push(format!("from_rir failed for value {}: {}", value, e));
            }
        }

        if !failures.is_empty() {
            panic!(
                "Test failed with the following errors:\n{}",
                failures.join("\n")
            );
        }
    }
}
