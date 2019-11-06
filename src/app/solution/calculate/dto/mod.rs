use serde::{Deserialize};
use crate::app::solution::calculate::Vec2d_i32;

#[derive(Deserialize, Debug)]
pub struct CalculateSolutionDto {
    pub supply: Vec<i32>,
    pub demand: Vec<i32>,
    pub costs: Vec2d_i32,
}