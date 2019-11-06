use crate::app::solution::calculate::UnsignedMap;
use crate::app::solution::calculate::cost::CostMap;

pub type DualVariables = Vec<Vec<Option<i32>>>;

pub fn calculate_dual_variables(base: &UnsignedMap, costs: &CostMap) -> DualVariables {
    let mut dual_variables: DualVariables = vec![vec![None; base.len()], vec![None; base[0].len()]];
    let node_count = get_node_count(base);
    let dual_variables_count = dual_variables[0].len() + dual_variables[1].len();
    if node_count < dual_variables_count {
        let n = std::cmp::min(dual_variables[0].len(), dual_variables_count - node_count);
        for i in 0..n {
            dual_variables[0][i] = Some(0);
            calculate_variables_for(&mut dual_variables, base, costs, 0, i);
        }
    } else {
        dual_variables[0][0] = Some(0);
        calculate_variables_for(&mut dual_variables, base, costs, 0, 0);
    }

    dual_variables
}

fn calculate_variables_for(variables: &mut DualVariables, solution: &UnsignedMap, costs: &CostMap, i: usize, j: usize) {
    if i == 0 {
        for k in 0..solution[j].len() {
            if solution[j][k] != 0 && variables[1][k].is_none() {
                variables[1][k] = Some(costs[j][k].value as i32 - variables[i][j].unwrap());
                calculate_variables_for(variables, solution, costs, 1, k);
            }
        }
    } else {
        for k in 0..solution.len() {
            if solution[k][j] != 0 && variables[0][k].is_none() {
                variables[0][k] = Some(costs[k][j].value as i32 - variables[i][j].unwrap());
                calculate_variables_for(variables, solution, costs, 0, k);
            }
        }
    }
}

fn get_node_count(base: &UnsignedMap) -> usize {
    let mut result = 0;
    for row in base.iter() {
        for it in row.iter() {
            if *it != 0 {
                result += 1;
            }
        }
    }
    result
}