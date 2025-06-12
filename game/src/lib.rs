mod board;
mod interface;

use std::{error::Error, time::Duration};
use sdl2::{
    event::Event,
    mouse::MouseButton,
    pixels::Color,
};
use board::Board;

const P1: char = 'X';
const P2: char = 'O';

#[inline]
const fn player_name(turn: bool) -> char {
    if turn { P1 } else { P2 }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut board = Board::default();
    let mut turn = true;

    let sdl = sdl2::init()?;
    let ttf = sdl2::ttf::init()?;
    let video = sdl.video()?;

    let window = video
        .window("Rusty Tetris", interface::SCREEN_SIZE, interface::SCREEN_SIZE)
        .position_centered()
        .build()?;

    let mut canvas = window
        .into_canvas()
        .build()?;

    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        // event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x, y,
                    ..
                } => {
                    let x = x as usize / interface::CELL_SIZE as usize;
                    let y = y as usize / interface::CELL_SIZE as usize;
                    println!("x, y: {}, {}", x, y);
                    let i = y * 3 + x;
                    match board[i] {
                        Some(_) => {
                            println!("Cell already occupied");
                            continue 'running;
                        }
                        None => {
                            board[i] = Some(turn);
                        }
                    }
                    if board.has_won(turn) {
                        println!("Player `{}` has won", player_name(turn));
                        break 'running;
                    }
                    if board.is_full() {
                        println!("It's a tie");
                        break 'running;
                    }
                    turn = !turn;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        interface::draw(&mut canvas, &ttf, &board)?;

        canvas.present();
        
        println!("turn : {}", turn);
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
