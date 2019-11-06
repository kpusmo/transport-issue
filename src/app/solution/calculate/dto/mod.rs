use serde::{Deserialize};
use crate::app::solution::calculate::UnsignedMap;

#[derive(Deserialize, Debug)]
pub struct CalculateSolutionDto {
    pub supply: Vec<u32>,
    pub demand: Vec<u32>,
    pub costs: UnsignedMap,
}