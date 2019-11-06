use actix_web::{HttpResponse, Responder, web};

use cycle_finder::Cycle;
use cycle_finder::CycleFinder;
use dto::CalculateSolutionDto;
use dual_variables::DualVariables;
use cost::{CostMap};
use response::SolutionResponse;

use crate::error::Error;

mod cycle_finder;
mod dual_variables;
mod base_result_min_nodes;
mod dto;
mod cost;
mod response;

type Vec2d_i32 = Vec<Vec<i32>>;
type Coords = (usize, usize);

pub fn index(mut dto: web::Json<CalculateSolutionDto>) -> impl Responder {
    let costs = CostMap::from(&mut dto);
    let base = base_result_min_nodes::base_min(&mut dto, &costs);
    let history = match optimize(base, &costs) {
        Ok(r) => r,
        Err(message) => return HttpResponse::InternalServerError().json(Error::InternalServerError(message))
    };
    HttpResponse::Ok().json(history)
}

fn optimize(base: Vec2d_i32, costs: &CostMap) -> Result<Vec<SolutionResponse>, String> {
    let mut result = vec![];
    let mut last_solution = base;
    loop {
        let dual_variables = dual_variables::calculate_dual_variables(&last_solution, costs);
        let delta = calculate_deltas(costs, &dual_variables);
        let cost = calculate_cost(&last_solution, costs);

        let min_delta_coords = min_element_coords(&delta);
        let min_delta_value = delta[min_delta_coords.0][min_delta_coords.1];

        result.push(SolutionResponse {
            solution: last_solution.clone(),
            cost,
            delta,
            dual_variables
        });

        if min_delta_value >= 0 {
            return Ok(result);
        }

        let mut cycle_finder = CycleFinder::new(min_delta_coords, &last_solution);
        let cycle = cycle_finder.find()?;
        let diff = min_in_cycle(&cycle, &last_solution);

        for (i, position) in cycle.iter().enumerate() {
            if i & 1 == 0 {
                last_solution[position.0][position.1] += diff;
            } else {
                last_solution[position.0][position.1] -= diff;
            }
        }
    }
}

fn min_in_cycle(cycle: &Cycle, map: &Vec2d_i32) -> i32 {
    let mut diff = None;
    for (i, position) in cycle.iter().enumerate() {
        // minimum of negative cycle part - odd elements
        if i & 1 == 0 {
            continue;
        }
        let value = map[position.0][position.1];
        if (diff.is_none() || value < diff.unwrap()) && value != 0 {
            diff = Some(value);
        }
    }
    diff.unwrap()
}

fn min_element_coords<T: Ord + Copy>(vec: &Vec<Vec<T>>) -> Coords {
    let mut min = None;
    let mut result = (0, 0);
    for (i, row) in vec.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if min.is_none() || *item < min.unwrap() {
                min = Some(*item);
                result.0 = i;
                result.1 = j;
            }
        }
    }
    result
}

fn calculate_deltas(costs: &CostMap, dual_variables: &DualVariables) -> Vec2d_i32 {
    let mut delta = vec![vec![0; costs[0].len()]; costs.len()];
    for (i, row) in costs.iter().enumerate() {
        for (j, it) in row.iter().enumerate() {
            delta[i][j] = it.value as i32 - dual_variables[0][i].unwrap() - dual_variables[1][j].unwrap();
        }
    }

    delta
}

fn calculate_cost(solution: &Vec2d_i32, costs: &CostMap) -> i32 {
    let mut result = 0;
    for i in 0..solution.len() {
        for j in 0..solution[i].len() {
            result += solution[i][j] * costs[i][j].value;
        }
    }
    result
}

