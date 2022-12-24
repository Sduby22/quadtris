use lazy_static::lazy_static;

use crate::enums::{PieceType, Rotation};

pub const R: isize = 20;
pub const C: isize = 10;

pub type Kick = (isize, isize);

pub(crate) static I_KICKS: [[Kick; 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];

pub(crate) static DEFAULT_KICKS: [[Kick; 5]; 8] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

lazy_static! {
    pub static ref I_MATRIX: [Vec<u8>; 4] = [
        vec![0, 15, 0, 0],
        vec![2, 2, 2, 2],
        vec![0, 0, 15, 0],
        vec![4, 4, 4, 4]
    ];
    pub static ref O_MATRIX: [Vec<u8>; 4] =
        [vec![6, 6, 0], vec![6, 6, 0], vec![6, 6, 0], vec![6, 6, 0],];
    pub static ref Z_MATRIX: [Vec<u8>; 4] =
        [vec![6, 3, 0], vec![1, 3, 2], vec![0, 6, 3], vec![2, 6, 4]];
    pub static ref S_MATRIX: [Vec<u8>; 4] =
        [vec![3, 6, 0], vec![2, 3, 1], vec![0, 3, 6], vec![4, 6, 2]];
    pub static ref J_MATRIX: [Vec<u8>; 4] =
        [vec![4, 7, 0], vec![3, 2, 2], vec![0, 7, 1], vec![2, 2, 6]];
    pub static ref L_MATRIX: [Vec<u8>; 4] =
        [vec![1, 7, 0], vec![2, 2, 3], vec![0, 7, 4], vec![6, 2, 2]];
    pub static ref T_MATRIX: [Vec<u8>; 4] =
        [vec![2, 7, 0], vec![2, 3, 2], vec![0, 7, 2], vec![2, 6, 2]];
}

pub fn get_rotations(piece: PieceType, rotation: Rotation) -> &'static Vec<u8> {
    piece!(
        piece,
        O => &O_MATRIX[rotation as usize],
        I => &I_MATRIX[rotation as usize],
        Z => &Z_MATRIX[rotation as usize],
        S => &S_MATRIX[rotation as usize],
        J => &J_MATRIX[rotation as usize],
        L => &L_MATRIX[rotation as usize],
        T => &T_MATRIX[rotation as usize],
    )
}
