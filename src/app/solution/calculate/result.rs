use crate::app::solution::calculate::{UnsignedMap, SignedMap};
use crate::app::solution::calculate::dual_variables::DualVariables;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalculationResult {
    pub solution: UnsignedMap,
    pub delta: SignedMap,
    pub dual_variables: DualVariables,
    pub cost: u32,
}