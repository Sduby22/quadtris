use crate::constants::*;
use macroquad::prelude::*;
use rust_tetris_core::{
    board::{Board, Cell},
    enums::{PieceType, PieceTypeColor},
};

use super::block::{BlockRenderer, BlockVariant};

pub fn render_board(board: &Board, block_renderer: &BlockRenderer) {
    board.rows().flatten().enumerate().for_each(|(ind, cell)| {
        let x = ind % board.cols as usize;
        let y = ind / board.cols as usize;

        render_cell(
            cell,
            Vec3 {
                x: BLOCK_SIZE * x as f32,
                y: BLOCK_SIZE * y as f32,
                z: 0.,
            },
            block_renderer,
        );
    })
}

fn render_cell(cell: &Cell, position: Vec3, block_renderer: &BlockRenderer) {
    if let Cell::Filled(piece) = cell {
        let variant = match piece {
            PieceTypeColor::Playable(piece_type) => match piece_type {
                PieceType::T => BlockVariant::PURPLE,
                PieceType::L => BlockVariant::BLUE,
                PieceType::J => BlockVariant::ORANGE,
                PieceType::S => BlockVariant::GREEN,
                PieceType::Z => BlockVariant::RED,
                PieceType::O => BlockVariant::YELLOW,
                PieceType::I => BlockVariant::CYAN,
            },
            PieceTypeColor::NotPlayable => BlockVariant::GRAY,
        };

        block_renderer.draw_block(variant, position, BLOCK_SIZE);
    }
}
