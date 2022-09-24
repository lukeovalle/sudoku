mod interface;
mod sudoku;
mod game;

fn main() {
    if let Err(e) = game::run() {
        eprintln!("{}", e);
    }
}


