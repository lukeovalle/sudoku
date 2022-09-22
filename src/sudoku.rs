use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O Error")]
    Io(#[source] std::io::Error),
    #[error("Created grid has an invalid size. rows: {rows:?}, columns: {cols:?}")]
    MismatchedGrid { rows: usize, cols: usize }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::Io(other)
    }
}


#[derive(Clone)]
pub enum Number {
    Empty,
    Given(u8),
    Answer(u8)
}
pub struct Sudoku {
    rows: Vec<Vec<Number>>
}

impl Sudoku {
    pub fn new(size: usize) -> Sudoku {
        let rows = vec![vec![Number::Empty;size];size];

        Sudoku { rows }
    }

    pub fn from_file(path: &Path) -> Result<Sudoku, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let rows: Vec<Vec<_>> = reader.lines().filter_map(|l| l.ok()).map( |line|
            line.split(',')
                .map(|n|
                    u8::from_str_radix(n, 10)
                        .and_then(|n|
                            Ok(Number::Given(n))
                        ).unwrap_or(Number::Empty)
                ).collect()
        ).collect();

        for row in &rows {
            if row.len() != rows.len() {
                return Err(
                    Error::MismatchedGrid{ rows: rows.len(), cols: row.len() }
                );
            }
        }
        
        let sudoku = Sudoku { rows };


        Ok(sudoku)
    }

    pub fn insert_number(&mut self, row: usize, col: usize, answer: u8) {
        if let Some(val) = self.rows[row].get_mut(col) {
            match val {
                Number::Empty | Number::Answer(..) => { *val = Number::Answer(answer); }
                Number::Given(..) => {}
            }
        }
    }

    pub fn check_position(&self, row: usize, col: usize) -> &Number {
        &self.rows[row][col]
    }

    pub fn iterate(&self) -> SudokuIter {
        SudokuIter::new(&self.rows)
    }
}

pub struct SudokuIter<'a> {
    sudoku: &'a Vec<Vec<Number>>,
    i: usize,
    j: usize
}

impl<'a> SudokuIter<'a> {
    fn new(sudoku: &Vec<Vec<Number>>) -> SudokuIter {
        SudokuIter { sudoku, i: 0, j: 0 }
    }
}

impl<'a> Iterator for SudokuIter<'a> {
    type Item = (usize, usize, &'a Number);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.sudoku.get(self.i) {
            if let Some(val) = row.get(self.j) {
                self.j += 1;

                return Some((self.i, self.j - 1,  val));
            } else {
                self.i += 1;
                self.j = 0;

                return self.next();
            }
        } else {
            return None;
        }
    }
}
