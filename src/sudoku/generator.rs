use crate::sudoku::*;
use rand::Rng;

impl Sudoku {
   pub fn generate(size: usize) -> Sudoku {
       let mut rng = rand::thread_rng();
       let mut sudoku = Sudoku::new(size);

       for _ in 0..size*size/2 {
           let i = rng.gen_range(0..size);
           let j = rng.gen_range(0..size);
           let n = rng.gen_range(1..=size);

           sudoku.insert_given(i, j, n as u8);
       }

       sudoku
   }
}

