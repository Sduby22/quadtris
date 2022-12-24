use crate::{constants::*, game_data::GameData};
use macroquad::prelude::*;
use rust_tetris_core::{
    board::{Board, Cell},
    enums::{PieceType, PieceTypeColor},
    pieces::{Piece, PieceWithPosition},
};

use super::{
    block::{BlockRenderer, BlockVariant},
    text::{self, TextRenderer},
    utils::{pop_model_matrix, push_model_matrix},
};
use lazy_static::lazy_static;

pub fn render_board(
    game_data: &GameData,
    pos: Vec3,
    block_renderer: &BlockRenderer,
    text_renderer: &TextRenderer,
) {
    let board = &game_data.board;
    push_model_matrix(Mat4::from_translation(pos));
    render_frame(board.rows, board.cols, BLOCK_SIZE);

    render_board_blocks(board, block_renderer);
    render_next(game_data, text_renderer, block_renderer);
    render_hold(game_data, text_renderer, block_renderer);

    let t = PieceWithPosition::new(0, 0, Piece::new(PieceType::I));
    render_tetrimino(&t, block_renderer);

    pop_model_matrix();
}

fn render_next(game_data: &GameData, text_renderer: &TextRenderer, block_renderer: &BlockRenderer) {
    push_model_matrix(*NEXT_MAT);

    text_renderer.draw_text(
        "NEXT",
        Vec2 {
            x: 0.,
            y: BLOCK_SIZE * 2.,
        },
        FONT_SIZE,
        text::Color::YELLOW,
    );

    let (mut dx, mut dy) = (1, -1);
    game_data
        .piece_bag
        .piece_buffer()
        .iter()
        .enumerate()
        .for_each(|(i, piece)| {
            render_tetrimino(
                &PieceWithPosition::new(dy, dx, piece.clone()),
                block_renderer,
            );
            if i < 2 {
                dx += 1 + piece.width();
            } else {
                dy -= 1 + piece.height();
            }
        });

    pop_model_matrix();
}

fn render_hold(game_data: &GameData, text_renderer: &TextRenderer, block_renderer: &BlockRenderer) {
    push_model_matrix(*HOLD_MAT);

    text_renderer.draw_text(
        "HOLD",
        Vec2 {
            x: 0.,
            y: BLOCK_SIZE * 2.,
        },
        FONT_SIZE,
        text::Color::YELLOW,
    );

    if let Some(hp) = &game_data.hold_piece {
        render_tetrimino(
            &PieceWithPosition::new(-1, 1, hp.piece.clone()),
            block_renderer,
        )
    }

    pop_model_matrix();
}

fn render_board_blocks(board: &Board, block_renderer: &BlockRenderer) {
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
    });
}

fn render_tetrimino(tetrimino: &PieceWithPosition, block_renderer: &BlockRenderer) {
    push_model_matrix(Mat4::from_translation(Vec3 {
        x: tetrimino.col() as f32 * BLOCK_SIZE,
        y: tetrimino.row() as f32 * BLOCK_SIZE,
        z: 0.,
    }));
    render_board_blocks(&tetrimino.tetris_piece_ref().board, block_renderer);
    pop_model_matrix();
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

fn render_frame(rows: isize, cols: isize, size: f32) {
    let offset = size / 2.;
    (0..=rows).for_each(|row| {
        let y = row as f32 * size - offset;
        let x = cols as f32 * size - offset;

        draw_line_3d(
            Vec3 {
                x: -offset,
                y,
                z: -offset,
            },
            Vec3 { x, y, z: -offset },
            Color::from_rgba(255, 255, 255, 128),
        );
    });

    (0..=cols).for_each(|col| {
        let x = col as f32 * size - offset;
        let y = rows as f32 * size - offset;

        draw_line_3d(
            Vec3 {
                x,
                y: -offset,
                z: -offset,
            },
            Vec3 { x, y, z: -offset },
            Color::from_rgba(255, 255, 255, 128),
        );
    });
}

lazy_static! {
    static ref HOLD_MAT: Mat4 = Mat4::from_translation(Vec3 {
        x: 0.,
        y: 20. * BLOCK_SIZE,
        z: 0.,
    }) * Mat4::from_scale(Vec3::splat(0.6));
    static ref NEXT_MAT: Mat4 = Mat4::from_translation(Vec3 {
        x: 4.5 * BLOCK_SIZE,
        y: 20. * BLOCK_SIZE,
        z: 0.,
    }) * Mat4::from_scale(Vec3::splat(0.6));
}
