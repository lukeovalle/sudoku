//use thiserror::Error;
use std::time::{Duration, Instant};
use std::collections::hash_set::HashSet;
use crate::interface;
use crate::sudoku;
use interface::{Action, Direction};
use sudoku::{Sudoku, Number};

pub fn run() -> Result<(), anyhow::Error> {
//    sdl2::hint::set("SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let mut game_context = interface::initialize_sdl()?;
    let mut sudoku = Sudoku::from_file(std::path::Path::new("tests/example"))?;
    let time_per_frame = Duration::new(1, 0) / 60;
    // (row, col)
    let mut selection: Option<(usize, usize)> = None;
    let mut redraw = true;
    let mut errors = HashSet::new();

    'game: loop {
        let now = Instant::now();

        // process input
        let (x, y) = game_context.canvas.window().drawable_size();
        match interface::check_input(
            &mut game_context.event_pump,
            x.try_into()?,
            y.try_into()?
        ) {
            Some(Action::Quit) => {
                break 'game;
            }
            Some(Action::Redraw) => {
                redraw = true;
            }
            Some(Action::Select { row, col } ) => {
                selection = Some((row, col));
                redraw = true;
            }
            Some(Action::MoveSelection { dir } ) => {
                selection = move_selection(selection, dir);

                redraw = true;
            }
            Some(Action::Insert { number } ) => {
                // if there is something selected
                if let Some((row, col)) = selection {
                    // if it's not a given number
                    match sudoku.check_position(row, col) {
                        Number::Empty | Number::Answer(_) => {
                            sudoku.insert_number(row, col, number);
                            redraw = true;
                        }
                        _ => {}
                    }
                }
            }
            Some(Action::Delete) => {
                if let Some((row, col)) = selection {
                    sudoku.delete_number(row, col);
                    redraw = true;
                }
            }
            Some(Action::Solve) => {
                if let Some(sol) = sudoku.solve() {
                    sudoku = sol;
                    redraw = true;
                }
            }
            Some(Action::Check) => {
                errors = sudoku.check_rules();
                redraw = true;
            }
            None => {}
        }

        // run logic


        // render
        if redraw {
            if let Err(e) = interface::render_window(
                &mut game_context,
                &sudoku,
                &selection,
                &errors
            ) {
                eprintln!("{}", e);
            }
            redraw = false;
        }

        ::std::thread::sleep(time_per_frame.saturating_sub(now.elapsed()));
    }

    Ok(())
}

fn move_selection(
    selection: Option<(usize, usize)>,
    direction: Direction
) -> Option<(usize, usize)> {
    selection.map({ |(row, col)|
        match direction {
            Direction::Up => (row.saturating_sub(1), col),
            Direction::Down => ((row + 1).min(8), col),
            Direction::Left => (row, col.saturating_sub(1)),
            Direction::Right => (row, (col + 1).min(8))
        }
    })
}

