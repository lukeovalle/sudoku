use std::collections::hash_set::HashSet;

use crate::sudoku::*;

impl Sudoku {
    pub fn solve(&self) -> Option<Sudoku> {
        let mut solutions = Vec::new();
        let now = std::time::Instant::now();

        self.recursive_solve(0, 0, &mut solutions);

        let elapsed = now.elapsed();
        println!("solved in {}.{:03}s.",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );

        solutions.first().map( |sol| sol.clone() )
    }

    fn recursive_solve(
        &self,
        row: usize,
        col: usize,
        solutions: &mut Vec<Sudoku>
    ) {
        let size = self.size() as u8;

        let mut aux = self.clone();
        aux.solve_by_naked_singles();

        match aux.check_position(row, col) {
            Number::Answer(_) | Number::Given(_) => {
                aux.next_recursion(row, col, solutions);
            }
            Number::Empty => {
                aux.solve_by_naked_singles();
                for i in 1..=size {
                    aux.insert_number(row, col, i);

                    if !aux.check_rules().is_empty() {
                        continue;
                    }

                    aux.next_recursion(row, col, solutions);
                }
            }
        }
    }

    fn next_recursion(
        &self,
        row: usize,
        col: usize,
        solutions: &mut Vec<Sudoku>
    ) {
        let size = self.size();

        // if it reaches the end of the sudoku, push the solution
        if size - 1 == row && size - 1 == col {
            solutions.push(self.clone());
        } else {
            if col == size - 1 {
                self.recursive_solve(row + 1, 0, solutions);
            } else {
                self.recursive_solve(row, col + 1, solutions);
            }
        }
    }

    pub fn check_rules(&self) -> HashSet<(usize, usize)> {
        let mut wrong_numbers = HashSet::new();
        let size = self.size();

        for i in 0..size {
            for j in 0..size {
                self.check_row(i, j, &mut wrong_numbers);
                self.check_column(i, j, &mut wrong_numbers);
                self.check_box(i, j, &mut wrong_numbers);
            }
        }

        wrong_numbers
    }

    fn check_row(
        &self,
        row: usize,
        col: usize,
        wrong_numbers: &mut HashSet<(usize, usize)>
    ) {
        let pos = self.check_position(row, col);

        for i in 0..self.size() {
            if col == i {
                continue;
            }

            if pos.compare(self.check_position(row, i)) {
                wrong_numbers.insert((row, col));
                wrong_numbers.insert((row, i));
            }
        }
    }

    fn check_column(
        &self,
        row: usize,
        col: usize,
        wrong_numbers: &mut HashSet<(usize, usize)>
    ) {
        let pos = self.check_position(row, col);

        for i in 0..self.size() {
            if row == i {
                continue;
            }

            if pos.compare(self.check_position(i, col)) {
                wrong_numbers.insert((row, col));
                wrong_numbers.insert((i, col));
            }
        }
    }

    fn check_box(
        &self,
        row: usize,
        col: usize,
        wrong_numbers: &mut HashSet<(usize, usize)>
    ) {
        let pos = self.check_position(row, col);

        for i in (row / 3 * 3)..(row / 3 * 3 + 3) {
            for j in (col / 3 * 3)..(col / 3 * 3 + 3) {
                if row == i && col == j {
                    continue;
                }

                if pos.compare(self.check_position(i, j)) {
                    wrong_numbers.insert((row, col));
                    wrong_numbers.insert((i, j));
                }
            }
        }
    }

    fn solve_by_naked_singles(&mut self) {
        let total_numbers = HashSet::from_iter(1..=(self.size() as u8));
        let mut const_row: Vec<_> = (0..self.size()).map(|i| self.constraints_row(i)).collect();
        let mut const_col: Vec<_> = (0..self.size()).map(|i| self.constraints_col(i)).collect();
        let mut const_box: Vec<_> = (0..self.size()).map(|i| self.constraints_box(i)).collect();

        let size = self.size();
        let mut stop = false;

        while stop == false {
            stop = true;
            for i in 0..size {
                for j in 0..size {
                    if self.check_position(i, j) != Number::Empty {
                        continue;
                    }

                    let box_number = self.box_number(i, j);

                    let possible = &total_numbers - &const_row[i];
                    let possible = &possible - &const_col[j];
                    let possible = &possible - &const_box[box_number];

                    if possible.len() != 1 {
                        continue;
                    }

                    for number in possible.iter() {
                        self.insert_number(i, j, *number);
                        const_row[i] = self.constraints_row(i);
                        const_col[j] = self.constraints_col(j);
                        const_box[box_number] = self.constraints_box(box_number);
                        stop = false; // Only stop if there is nothing to insert
                    }
                }
            }
        }
    }

    fn constraints_row(&self, row: usize) -> HashSet<u8> {
        let mut constraints = HashSet::with_capacity(self.size());

        for i in 0..self.size() {
            match self.check_position(row, i) {
                Number::Empty => continue,
                Number::Answer(n) | Number::Given(n) => {
                    constraints.insert(n);
                }
            }
        }

        constraints
    }

    fn constraints_col(&self, col: usize) -> HashSet<u8> {
        let mut constraints = HashSet::with_capacity(self.size());

        for i in 0..self.size() {
            match self.check_position(i, col) {
                Number::Empty => continue,
                Number::Answer(n) | Number::Given(n) => {
                    constraints.insert(n);
                }
            }
        }

        constraints
    }

    fn constraints_box(&self, box_: usize) -> HashSet<u8> {
        let mut constraints = HashSet::with_capacity(self.size());

        for i in (box_ / 3 * 3)..(box_ / 3 * 3 + 3) { // rows
            for j in (box_ % 3 * 3)..(box_ % 3 * 3 + 3) { // columns
                match self.check_position(i, j) {
                    Number::Empty => continue,
                    Number::Answer(n) | Number::Given(n) => {
                        constraints.insert(n);
                    }
                }
            }
        }

        constraints
    }
}
