use crate::app::solution::calculate::{Coords, UnsignedMap};

pub type Cycle = Vec<Coords>;

pub struct CycleFinder<'a> {
    start_position: (usize, usize),
    current_position: (usize, usize),
    horizontal: bool,
    map: &'a UnsignedMap,
}

// interface
impl<'a> CycleFinder<'a> {
    pub fn new(start_position: (usize, usize), map: &'a UnsignedMap) -> CycleFinder {
        CycleFinder {
            start_position,
            current_position: start_position,
            horizontal: false,
            map,
        }
    }

    pub fn find(&mut self) -> Result<Cycle, String> {
        let mut cycle = vec![self.start_position];
        self.cycle_step(&mut cycle);
        return if cycle.len() < 4 {
            Err("Brak cyklu".to_string())
        } else {
            Ok(cycle)
        }
    }
}

impl<'a> CycleFinder<'a> {
    fn cycle_step(&mut self, cycle: &mut Cycle) {
        if self.can_stop(cycle) {
            return;
        }
        let size = if self.horizontal {
            self.map.len()
        } else {
            self.map[self.current_position.0].len()
        };

        for i in 0..size {
            let position = self.get_position(i);
            let value = self.map[position.0][position.1];
            if value == 0 || cycle.contains(&position) {
                continue;
            }

            let temp = self.current_position;
            self.current_position = position;
            self.horizontal = !self.horizontal;
            cycle.push(position);
            self.cycle_step(cycle);

            if self.can_stop(cycle) {
                return;
            }
            self.horizontal = !self.horizontal;
            self.current_position = temp;
            cycle.pop();
        }
    }

    fn get_position(&self, i: usize) -> (usize, usize) {
        if self.horizontal {
            (i, self.current_position.1)
        } else {
            (self.current_position.0, i)
        }
    }

    fn can_stop(&self, cycle: &Cycle) -> bool {
        let len = cycle.len();
        len > 3
            && len & 1 == 0
            && (
            (self.horizontal && self.current_position.1 == self.start_position.1)
                || (!self.horizontal && self.current_position.0 == self.start_position.0)
        )
    }
}