use std::ops::{Index, IndexMut};

use crate::app::solution::calculate::dto::CalculateSolutionDto;
use crate::app::solution::calculate::Vec2d_i32;

#[derive(Clone, Copy, Debug)]
pub struct Cost {
    pub value: i32,
    pub virtual_value: i32,
}

#[derive(Default, Debug)]
pub struct CostMap {
    values: Vec<Vec<Cost>>,
}

impl CostMap {
    pub fn from(dto: &mut CalculateSolutionDto) -> CostMap {
        let mut instance = CostMap {
            values: vec![vec![]; dto.costs.len()],
        };

        for (i, row) in dto.costs.iter().enumerate() {
            for cost in row.iter() {
                instance.values[i].push(Cost {
                    value: *cost,
                    virtual_value: *cost,
                })
            }
        }

        let supply: i32 = dto.supply.iter().sum();
        let demand: i32 = dto.demand.iter().sum();
        if supply == demand {
            return instance;
        }

        if supply > demand {
            instance.add_virtual_receiver(dto);
            dto.demand.push(supply - demand);
        } else {
            instance.add_virtual_supplier(dto);
            dto.supply.push(demand - supply);
        }

        instance
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Vec<Cost>> {
        self.values.iter()
    }

    fn add_virtual_receiver(&mut self, dto: &CalculateSolutionDto) {
        let virtual_cost = max_element(&dto.costs) * 100;
        for row in self.values.iter_mut() {
            row.push(Cost {
                value: 0,
                virtual_value: virtual_cost,
            })
        }
    }

    fn add_virtual_supplier(&mut self, dto: &CalculateSolutionDto) {
        let virtual_cost = max_element(&dto.costs) * 100;
        self.values.push(vec![
            Cost {
                value: 0,
                virtual_value: virtual_cost,
            }; dto.costs[0].len()
        ])
    }
}

impl Index<usize> for CostMap {
    type Output = Vec<Cost>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

impl IndexMut<usize> for CostMap {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.values[i]
    }
}

fn max_element(costs: &Vec2d_i32) -> i32 {
    let mut min = None;
    for row in costs.iter() {
        for item in row.iter() {
            if min.is_none() || *item < min.unwrap() {
                min = Some(*item);
            }
        }
    }
    min.unwrap()
}