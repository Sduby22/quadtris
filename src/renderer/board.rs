use crate::{constants::*, game_data::GameData};
use macroquad::prelude::*;
use rust_tetris_core::{
    board::{Board, Cell},
    enums::{PieceType, PieceTypeColor},
    pieces::PieceWithPosition,
};

use super::{
    block::{BlockRenderer, BlockVariant},
    text::{self, TextRenderer},
    utils::{pop_model_matrix, push_model_matrix},
};
use lazy_static::lazy_static;

const HOLD_NEXT_PRIMARY_SCALE: f32 = 0.75;
const HOLD_NEXT_SECONDARY_SCALE: f32 = 0.5;

pub fn render_board(
    game_data: &GameData,
    pos: Vec3,
    block_renderer: &BlockRenderer,
    text_renderer: &TextRenderer,
) {
    let board = &game_data.board;
    push_model_matrix(Mat4::from_translation(pos));

    render_board_blocks(board, BLOCK_SIZE, block_renderer);
    render_curr_piece(game_data, block_renderer);
    render_ghost_piece(game_data, block_renderer);

    render_next(game_data, text_renderer, block_renderer);
    render_hold(game_data, text_renderer, block_renderer);

    render_frame(20, 10, BLOCK_SIZE);

    pop_model_matrix();
}

fn render_ghost_piece(game_data: &GameData, block_renderer: &BlockRenderer) {
    if let Some(mut p) = game_data.curr_piece.clone() {
        while !p.collides_down(&game_data.board) {
            p.move_down();
        }

        render_tetrimino_wire(&p, BLOCK_SIZE, block_renderer);
    }
}

fn render_curr_piece(game_data: &GameData, block_renderer: &BlockRenderer) {
    if let Some(p) = &game_data.curr_piece {
        render_tetrimino(p, BLOCK_SIZE, block_renderer);
    }
}

fn render_next(game_data: &GameData, text_renderer: &TextRenderer, block_renderer: &BlockRenderer) {
    push_model_matrix(*NEXT_MAT);

    text_renderer.draw_text(
        "NEXT",
        Vec2 {
            x: BLOCK_SIZE * -0.2,
            y: BLOCK_SIZE * 2.5,
        },
        FONT_SIZE*HOLD_NEXT_SECONDARY_SCALE,
        text::Color::Yellow,
    );

    let (mut dx, mut dy) = (0, 0);

    if let Some(piece_bag) = &game_data.piece_bag {
        piece_bag
            .piece_buffer()
            .iter()
            .rev()
            .enumerate()
            .for_each(|(i, piece)| {
                render_tetrimino(
                    &PieceWithPosition::new(dy, dx, piece.clone()),
                    if i == 0 {
                        dx += 3;
                        BLOCK_SIZE*HOLD_NEXT_PRIMARY_SCALE
                    } else {
                        BLOCK_SIZE*HOLD_NEXT_SECONDARY_SCALE
                    },
                    block_renderer,
                );
                if i < 2 {
                    dx += 1 + get_piece_visual_size(piece.piece_type).0;
                } else {
                    dy -= 1 + get_piece_visual_size(piece.piece_type).1;
                }
            });
    }

    pop_model_matrix();
}

fn get_piece_visual_size(piece: PieceType) -> (isize, isize) {
    match piece {
        PieceType::I => (4, 2),
        _ => (3, 3),
    }
}

fn render_hold(game_data: &GameData, text_renderer: &TextRenderer, block_renderer: &BlockRenderer) {
    push_model_matrix(*HOLD_MAT);

    text_renderer.draw_text(
        "HOLD",
        Vec2 {
            x: BLOCK_SIZE * -0.2,
            y: BLOCK_SIZE * 2.5,
        },
        FONT_SIZE*HOLD_NEXT_SECONDARY_SCALE,
        text::Color::Yellow,
    );

    if let Some(hp) = &game_data.hold_piece {
        render_tetrimino(
            &PieceWithPosition::new(0, 0, hp.piece.clone()),
            BLOCK_SIZE*HOLD_NEXT_PRIMARY_SCALE,
            block_renderer,
        )
    }

    pop_model_matrix();
}

fn render_board_blocks(board: &Board, block_size: f32, block_renderer: &BlockRenderer) {
    board.rows().flatten().enumerate().for_each(|(ind, cell)| {
        let x = ind % board.cols as usize;
        let y = ind / board.cols as usize;

        render_cell(
            cell,
            Vec3 {
                x: block_size * x as f32,
                y: block_size * y as f32,
                z: 0.,
            },
            block_size,
            block_renderer,
        );
    });
}

fn render_board_blocks_wire(board: &Board, block_size: f32, block_renderer: &BlockRenderer) {
    board
        .rows()
        .flatten()
        .enumerate()
        .filter(|(_, cell)| cell.is_filled())
        .for_each(|(ind, _)| {
            let x = ind % board.cols as usize;
            let y = ind / board.cols as usize;

            block_renderer.draw_wire_block(
                Vec3 {
                    x: block_size * x as f32,
                    y: block_size * y as f32,
                    z: 0.,
                },
                block_size,
            );
        });
}

fn render_tetrimino(tetrimino: &PieceWithPosition, block_size: f32, block_renderer: &BlockRenderer) {
    push_model_matrix(Mat4::from_translation(Vec3 {
        x: tetrimino.col() as f32 * block_size,
        y: tetrimino.row() as f32 * block_size,
        z: 0.,
    }));
    render_board_blocks(&tetrimino.tetris_piece_ref().board, block_size, block_renderer);
    pop_model_matrix();
}

fn render_tetrimino_wire(tetrimino: &PieceWithPosition, block_size: f32, block_renderer: &BlockRenderer) {
    push_model_matrix(Mat4::from_translation(Vec3 {
        x: tetrimino.col() as f32 * block_size,
        y: tetrimino.row() as f32 * block_size,
        z: 0.,
    }));
    render_board_blocks_wire(&tetrimino.tetris_piece_ref().board, block_size, block_renderer);
    pop_model_matrix();
}

fn render_cell(cell: &Cell, position: Vec3, size: f32, block_renderer: &BlockRenderer) {
    if let Cell::Filled(piece) = cell {
        let variant = match piece {
            PieceTypeColor::Playable(piece_type) => match piece_type {
                PieceType::T => BlockVariant::Purple,
                PieceType::L => BlockVariant::Blue,
                PieceType::J => BlockVariant::Orange,
                PieceType::S => BlockVariant::Green,
                PieceType::Z => BlockVariant::Red,
                PieceType::O => BlockVariant::Yellow,
                PieceType::I => BlockVariant::Cyan,
            },
            PieceTypeColor::NotPlayable => BlockVariant::Gray,
        };

        block_renderer.draw_block(variant, position, size);
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
        x: 1. * BLOCK_SIZE,
        y: 19.5 * BLOCK_SIZE,
        z: 2. * BLOCK_SIZE,
    });
    static ref NEXT_MAT: Mat4 = Mat4::from_translation(Vec3 {
        x: 5.5 * BLOCK_SIZE,
        y: 19.5 * BLOCK_SIZE,
        z: 2. * BLOCK_SIZE,
    });
}
