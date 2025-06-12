#![allow(dead_code)] // TODO remove
use crate::board::Board;
use std::error::Error;
use sdl2::{
    pixels::Color,
    rect::{
        Rect,
        Point,
    },
    render::{
        Canvas,
        TextureQuery
    },
    ttf::Sdl2TtfContext,
    video::Window,
};
use crate::player_name;

pub(super) const CELL_SIZE: u32 = 200;
pub(super) const LINE_MARGIN: u32 = 30;
pub(super) const SCREEN_SIZE: u32 = CELL_SIZE * Board::SIZE as u32;


pub(super) fn draw(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, board: &Board) -> Result<(), Box<dyn Error>> {
    // matrix
    canvas.set_draw_color(Color::WHITE);
    for i in 1..Board::SIZE {
        let known_coord = i as u32 * CELL_SIZE;
        // horizontal
        canvas.draw_line(
            Point::new(LINE_MARGIN as i32, known_coord as i32),
            Point::new((SCREEN_SIZE - LINE_MARGIN) as i32, known_coord as i32)
        )?;
        // vertical
        canvas.draw_line(
            Point::new(known_coord as i32, LINE_MARGIN as i32),
            Point::new(known_coord as i32, (SCREEN_SIZE - LINE_MARGIN) as i32)
        )?;
    }
    

    // TODO draw pieces
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let Some(player) = cell else { continue; };
            let player = player_name(*player);
            text(
                canvas,
                ttf,
                x as i32 * CELL_SIZE as i32 + ((LINE_MARGIN * 7 / 6) as i32 / if player == 'X' { 1 } else { 2 }),
                y as i32 * CELL_SIZE as i32,
                &player.to_string(),
                CELL_SIZE as u16,
                "/System/Library/Fonts/Supplemental/Arial.ttf"
            )?;
        }
    }

    Ok(())
}

fn text(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, x: i32, y: i32, text: &str, text_size: u16, font_path: &str) -> Result<(), Box<dyn Error>> {
    let texture_creator = canvas.texture_creator();

    let font = ttf.load_font(font_path, text_size)?;

    let surface = font
        .render(text)
        .blended(Color::WHITE)?;

    let texture = texture_creator.create_texture_from_surface(&surface)?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}
