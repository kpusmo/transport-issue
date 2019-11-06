use std::cmp::min;
use crate::app::solution::calculate::Vec2d_i32;
use crate::app::solution::calculate::Coords;
use crate::app::solution::calculate::dto::CalculateSolutionDto;
use crate::app::solution::calculate::cost::CostMap;

pub fn base_min(dto: &mut CalculateSolutionDto, costs: &CostMap) -> Vec2d_i32 {
    let mut solution: Vec<Vec<Option<i32>>> = vec![vec![None; costs[0].len()]; costs.len()];
    let size = solution[0].len() * solution.len();
    let mut filled_count = 0;
    while filled_count < size {
        let min_cost_coords = min_cost_coords(&costs, &solution);
        let min_val = min(dto.supply[min_cost_coords.0], dto.demand[min_cost_coords.1]);
        solution[min_cost_coords.0][min_cost_coords.1] = Some(min_val);
        filled_count += 1;
        dto.supply[min_cost_coords.0] -= min_val;
        dto.demand[min_cost_coords.1] -= min_val;
        let count = fill_zeros(&mut solution, &min_cost_coords, dto.supply[min_cost_coords.0] == 0);
        filled_count += count;
    }
    let mut result = vec![vec![0; solution[0].len()]; solution.len()];
    for i in 0..solution.len() {
        for j in 0..solution[i].len() {
            result[i][j] = solution[i][j].unwrap();
        }
    }
    result
}

fn fill_zeros(vec: &mut Vec<Vec<Option<i32>>>, indices: &Coords, column: bool) -> usize {
    let mut result = 0;
    if column {
        for i in 0..vec[indices.0].len() {
            if vec[indices.0][i].is_none() {
                vec[indices.0][i] = Some(0);
                result += 1;
            }
        }
    } else {
        for i in 0..vec.len() {
            if vec[i][indices.1].is_none() {
                vec[i][indices.1] = Some(0);
                result += 1;
            }
        }
    }
    result
}

/**
    Finds min element in costs map, considering only positions that are is_none in map (that is base result).
    costs and map must be of the same shape.
*/
fn min_cost_coords<T: Ord + Copy>(costs: &CostMap, map: &Vec<Vec<Option<T>>>) -> Coords {
    let mut min = None;
    let mut result = (0, 0);
    for (i, row) in costs.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if map[i][j].is_none() && (min.is_none() || item.virtual_value < min.unwrap()) {
                min = Some(item.virtual_value);
                result.0 = i;
                result.1 = j;
            }
        }
    }
    result
}