use crate::sudoku::*;

impl Sudoku {
    pub fn solve(&self) -> Option<Sudoku> {
        let mut solutions = Vec::new();

        let now = std::time::Instant::now();
        self.recursive_solve(0, 0, &mut solutions);
        let elapsed = now.elapsed();
        println!("resuelto en {}.{}s", elapsed.as_secs(), elapsed.subsec_millis());

        if let Some(solution) = solutions.first() {
            Some(solution.clone())
        } else {
            None
        }
    }

    fn recursive_solve(&self, row: usize, col: usize, solutions: &mut Vec<Sudoku>) {
        let size = self.size() as u8;

        match self.check_position(row, col) {
            Number::Answer(_) | Number::Given(_) => {
                self.next_recursion(row, col, solutions);
            }
            Number::Empty => {
                let mut aux = self.clone();
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

    pub fn check_rules(&self) -> Vec<(usize, usize)> {
        let mut wrong_numbers = Vec::new();
        let size = self.size();

        // Check rows
        for i in 0..size {
            for j in 0..size {
                for k in (j+1)..size {
                    let pos = self.check_position(i, j);

                    if pos.compare(self.check_position(i, k)) {
                        wrong_numbers.push((i, j));
                        wrong_numbers.push((i, k));
                    }
                }
            }
        }

        // Check columns
        for i in 0..size {
            for j in 0..size {
                for k in (i+1)..size {
                    let pos = self.check_position(i, j);

                    if pos.compare(self.check_position(k, j)) {
                        wrong_numbers.push((i, j));
                        wrong_numbers.push((k, j));
                    }

                }
            }
        }
        
        // Check boxes
        for i in 0..size {
            for j in 0..size {
                // loop inside the box
                for a in (i / 3 * 3)..(i / 3 * 3 + 3) {
                    for b in (j / 3 * 3)..(j / 3 * 3 + 3) {
                        if i == a && j == b { continue }

                        let pos = self.check_position(i, j);

                        if pos.compare(self.check_position(a, b)) {
                            wrong_numbers.push((i, j));
                            wrong_numbers.push((a, b));
                        }
                    }
                }
            }
        }


        wrong_numbers
    }
}
