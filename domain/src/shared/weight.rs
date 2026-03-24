use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weight(u32);

#[derive(Debug, Error)]
pub enum WeightError {
    #[error("A weight below zero is not allowed")]
    Negative,
    #[error("A weight value above 5000 is not allowed")]
    TooLarge,
}

impl Weight {
    pub fn from_kg(kg: f32) -> Result<Self, WeightError> {
        match kg {
            ..0f32 => Err(WeightError::Negative),
            5001f32.. => Err(WeightError::TooLarge),
            _ => Ok(Weight((kg * 1000.0).round() as u32)),
        }
    }

    pub fn from_lbs(lbs: f32) -> Result<Self, WeightError> {
        match lbs {
            ..0f32 => Err(WeightError::Negative),
            5001f32.. => Err(WeightError::TooLarge),
            _ => Ok(Weight((lbs * 453.592).round() as u32)),
        }
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

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} grams", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_negative_weight() {
        let mut failures = Vec::new();

        for value in -5000i32..0i32 {
            if Weight::from_kg(value as f32).is_ok() {
                failures.push(format!(
                    "from_kg incorrectly succeeded for value: {}",
                    value
                ))
            }

            if Weight::from_lbs(value as f32).is_ok() {
                failures.push(format!(
                    "from_lbs incorrectly succeeded for value: {}",
                    value
                ))
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
    fn test_invalid_weight_greater_than_5000() {
        let mut failures = Vec::new();

        for value in 5001i32..7000i32 {
            if Weight::from_kg(value as f32).is_ok() {
                failures.push(format!(
                    "from_kg incorrectly succeeded for value: {}",
                    value
                ))
            }

            if Weight::from_lbs(value as f32).is_ok() {
                failures.push(format!(
                    "from_lbs incorrectly succeeded for value: {}",
                    value
                ))
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
    fn test_valid_weight_range_from_zero_to_5000() {
        let mut failures = Vec::new();

        for value in 0i32..=5000i32 {
            if let Err(e) = Weight::from_kg(value as f32) {
                failures.push(format!("from_kg failed for value {}: {}", value, e))
            }

            if let Err(e) = Weight::from_lbs(value as f32) {
                failures.push(format!("from_lbs failed for value {}: {}", value, e))
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
