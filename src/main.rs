mod interface;
mod sudoku;

//use thiserror::Error;
use std::time::{Duration, Instant};
use interface::Action;
use sudoku::Number;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}
//    sdl2::hint::set("SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

fn run() -> Result<(), anyhow::Error> {
    let mut game_context = interface::initialize_sdl()?;

    let mut sudoku = sudoku::Sudoku::from_file(std::path::Path::new("example"))?;

    let time_per_frame = Duration::new(1, 0) / 60;

    let mut selection: Option<(usize, usize)> = None;

    let mut redraw = true;

    'game: loop {
        let now = Instant::now();

        // process input
        let (x, y) = game_context.canvas.window().drawable_size();
        match interface::check_input(&mut game_context.event_pump, x.try_into()?, y.try_into()?) {
            Some(Action::Quit) => {
                break 'game;
            }
            Some(Action::Select { row, col } ) => {
                selection = Some((row, col));
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
            None => {}
        }

        // run logic


        // render
        if redraw {
            if let Err(e) = interface::render_window(&mut game_context, &sudoku, &selection) {
                dbg!("{}", e);
            }
            redraw = false;
        }

        ::std::thread::sleep(time_per_frame.saturating_sub(now.elapsed()));
    }

    Ok(())
}

