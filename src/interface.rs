use std::collections::HashMap;
use crate::sudoku::{Number, Sudoku};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use anyhow::anyhow;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub struct SdlContext {
    _sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    ttf_context: sdl2::ttf::Sdl2TtfContext
}

pub enum Action {
    Quit,
    Select { row: usize, col: usize },
    MoveSelection { dir: Direction },
    Insert { number: u8 },
    Delete,
    Redraw
}

#[derive(Clone, Copy)]
pub enum Direction { Up, Down, Left, Right }

pub fn initialize_sdl() -> Result<SdlContext, anyhow::Error> {
    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;

    let window = video_subsystem.window("Sudoku", 800, 800)
        .position_centered()
        //.resizable()
        .build()
        .map_err(|e| anyhow!(e))?;

    let canvas = window.into_canvas().build().map_err(|e| anyhow!(e))?;
    let event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;

    let ttf_context = sdl2::ttf::init()?;
    
    Ok(SdlContext {
        _sdl_context: sdl_context,
        _video_subsystem: video_subsystem,
        canvas,
        event_pump,
        ttf_context
    })
}

pub fn check_input(event_pump: &mut sdl2::EventPump, width: usize, length: usize) -> Option<Action> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Some(Action::Quit)
            }
            Event::MouseButtonDown {mouse_btn: MouseButton::Left, x, y, .. } => {
                return Some(Action::Select { 
                    row: x as usize * 9 / width,
                    col: y as usize * 9 / length
                })
            }
            Event::KeyDown { keycode: Some(Keycode::Delete), .. } => {
                return Some(Action::Delete)
            }
            Event::KeyDown { keycode: Some(key), .. } => {
                return check_input_numbers(&key)
                    .or(check_input_cursor(&key))
            }
            Event::Window { win_event: event, .. } => {
                return check_input_window_event(&event)
            }
            _ => {}
        }
    }

    None
}

fn check_input_numbers(key_pressed: &Keycode) -> Option<Action> {
    let keys: HashMap<Keycode, u8> = HashMap::from([
        (Keycode::Num1, 1),
        (Keycode::Kp1, 1),
        (Keycode::Num2, 2),
        (Keycode::Kp2, 2),
        (Keycode::Num3, 3),
        (Keycode::Kp3, 3),
        (Keycode::Num4, 4),
        (Keycode::Kp4, 4),
        (Keycode::Num5, 5),
        (Keycode::Kp5, 5),
        (Keycode::Num6, 6),
        (Keycode::Kp6, 6),
        (Keycode::Num7, 7),
        (Keycode::Kp7, 7),
        (Keycode::Num8, 8),
        (Keycode::Kp8, 8),
        (Keycode::Num9, 9),
        (Keycode::Kp9, 9)
    ]);

    keys.get(key_pressed).map(|n| Action::Insert { number: *n })
}

fn check_input_cursor(key_pressed: &Keycode) -> Option<Action> {
    let keys: HashMap<_, _> = HashMap::from([
        (Keycode::Up, Direction::Up),
        (Keycode::Down, Direction::Down),
        (Keycode::Right, Direction::Right),
        (Keycode::Left, Direction::Left)
    ]);

    keys.get(key_pressed).map(|dir| Action::MoveSelection { dir: *dir })
}

fn check_input_window_event(event: &WindowEvent) -> Option<Action> {
    match event {
        WindowEvent::Shown
        | WindowEvent::Exposed
        | WindowEvent::Maximized
        | WindowEvent::Restored
        | WindowEvent:: Enter
        | WindowEvent:: FocusGained
        | WindowEvent:: TakeFocus => Some(Action::Redraw),
        _ => None
    }
}

pub fn render_window(
    sdl: &mut SdlContext,
    sudoku: &Sudoku,
    selection: &Option<(usize, usize)>
) -> Result<(), anyhow::Error> {
    sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
    sdl.canvas.clear();

    let width = sdl.canvas.window().drawable_size().0.try_into()?;
    let height = sdl.canvas.window().drawable_size().1.try_into()?;
    let white = Color::WHITE;
    let grey = Color::GREY;

    // render grid
    render_grid(sdl, width, height, &white)?;

    // render numbers
    render_numbers(sdl, sudoku, width, height, &white, &grey)?;
    {
   }

    // selection rectangle
    let color = Color::RGB(30, 30, 220);
    render_selection_rectangle(sdl, selection, width, height, &color)?;

    sdl.canvas.present();

    Ok(())
}

fn render_grid(
    sdl: &mut SdlContext,
    width: i16,
    height: i16,
    color: &Color
) -> Result<(), anyhow::Error> {
    for i in 0..=9 {
        sdl.canvas.thick_line(
            i * width / 9,
            0,
            i * width / 9,
            height,
            if i % 3 == 0 { 6 } else { 2 },
            *color
        ).map_err(|e| anyhow!(e))?;

        sdl.canvas.thick_line(
            0,
            i * height / 9,
            width,
            i * height / 9,
            if i % 3 == 0 { 6 } else { 2 },
            *color
        ).map_err(|e| anyhow!(e))?;
    }

    Ok(())
}

fn render_selection_rectangle(
    sdl: &mut SdlContext,
    selection: &Option<(usize, usize)>,
    width: i16,
    height: i16,
    color: &Color
) -> Result<(), anyhow::Error> {
    if let Some((row, col)) = selection {
        let x_1 = (*row as i16) * width / 9;
        let x_2 = (*row as i16 + 1) * width / 9;
        let y_1 = (*col as i16) * height / 9;
        let y_2 = (*col as i16 + 1) * height / 9;
        
        sdl.canvas.thick_line(x_1, y_1, x_2, y_1, 6, *color).map_err(|e| anyhow!(e))?;
        sdl.canvas.thick_line(x_1, y_2, x_2, y_2, 6, *color).map_err(|e| anyhow!(e))?;
        sdl.canvas.thick_line(x_1, y_1, x_1, y_2, 6, *color).map_err(|e| anyhow!(e))?;
        sdl.canvas.thick_line(x_2, y_1, x_2, y_2, 6, *color).map_err(|e| anyhow!(e))?;
    }

    Ok(())
}

fn render_numbers(
    sdl: &mut SdlContext,
    sudoku: &Sudoku,
    width: i16,
    height: i16,
    color_input: &Color,
    color_given: &Color
) -> Result<(), anyhow::Error> {
    let font = sdl.ttf_context.load_font(
        "/usr/share/fonts/truetype/OpenSans-ExtraBold.ttf",
        (height / 9 - 10).try_into()?
    ).map_err(|e| anyhow!(e))?;

    for (i, j, number) in sudoku.iterate() {
        let pos_x = i as i32 * width as i32 / 9 + width as i32 / 18;
        let pos_y = j as i32 * height as i32 / 9 + height as i32 / 18;
        match number {
            Number::Answer(val) => {
                print_number(
                    &mut sdl.canvas,
                    &font,
                    (*val).try_into()?,
                    color_input,
                    (pos_x, pos_y)
                )?;

            }
            Number::Given(val) => {
                print_number(
                    &mut sdl.canvas,
                    &font,
                    (*val).try_into()?,
                    color_given,
                    (pos_x, pos_y)
                )?;
            }
            Number::Empty => {
            }
        }
    }

    Ok(())
}

fn print_number(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
    number: u32,
    color: &Color,
    position: (i32, i32)
) -> Result<(), anyhow::Error> {
    let c = std::char::from_digit(number.try_into()?, 10)
        .ok_or_else(|| "wtf").map_err(|e| anyhow!(e))?;

    let (w, h) = font.size_of_char(c)?;

    let surface = font.render(&c.to_string()).blended(*color)?;

    canvas.copy(
        &surface.as_texture(&canvas.texture_creator())?,
        None,
        sdl2::rect::Rect::from_center(
            sdl2::rect::Point::new(position.0, position.1),
            w,
            h
        )
    ).map_err(|e| anyhow!(e))?;

    Ok(())
}
