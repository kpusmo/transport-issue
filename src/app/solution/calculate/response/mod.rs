use crate::app::solution::calculate::{Vec2d_i32};
use crate::app::solution::calculate::dual_variables::DualVariables;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SolutionResponse {
    pub solution: Vec2d_i32,
    pub delta: Vec2d_i32,
    pub dual_variables: DualVariables,
    pub cost: i32,
}