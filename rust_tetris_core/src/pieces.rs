use crate::board::{playable_piece_to_cell, Board, Cell};
use crate::constants::{get_rotations, Kick, DEFAULT_KICKS, I_KICKS};
use crate::enums::{PieceType, PieceTypeColor, Rotation};
use std::fmt;

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub board: Board,
    pub rotation: Rotation,
}

impl Piece {
    pub fn with_rotation(piece_type: PieceType, rotation: Rotation) -> Self {
        let mut tetris_piece = Piece {
            piece_type,
            rotation,
            board: Board::new(0, 0),
        };

        tetris_piece.setup_board();

        tetris_piece
    }

    pub fn new(piece_type: PieceType) -> Self {
        Self::with_rotation(piece_type, Rotation::R0)
    }

    fn setup_board(&mut self) {
        self.board = get_piece_matrix(self.piece_type, self.rotation);
    }

    pub fn rotate_piece(&mut self) {
        self.set_rotation(self.rotation.next());
    }

    pub fn rotate_piece_prev(&mut self) {
        self.set_rotation(self.rotation.prev());
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
        self.setup_board();
    }

    pub fn width(&self) -> isize {
        self.board.cols
    }

    pub fn height(&self) -> isize {
        self.board.rows
    }

    pub fn all_cells(&self) -> impl Iterator<Item = (isize, isize)> {
        let w = self.width();
        let h = self.height();

        (0..h).flat_map(move |i| (0..w).map(move |j| (i, j)))
    }

    pub fn set_cells(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.all_cells()
            .filter(move |&(i, j)| self.board.is_set(i, j))
    }

    pub fn collides(&self, row: isize, col: isize, matrix: &Board) -> bool {
        self.all_cells().any(|(i, j)| {
            self.board.is_set(i, j)
                && (j + col < 0
                    || j + col >= matrix.cols
                    || i + row < 0
                    || i + row >= matrix.rows
                    || matrix.is_set(row + i, col + j))
        })
    }

    pub fn get_kicks(&self, from_rot: Rotation) -> &'static [Kick] {
        use Rotation::*;
        let kick_index = match (from_rot, self.rotation) {
            (R0, R90) => 0,
            (R90, R0) => 1,
            (R90, R180) => 2,
            (R180, R90) => 3,
            (R180, R270) => 4,
            (R270, R180) => 5,
            (R270, R0) => 6,
            (R0, R270) => 7,
            _ => unreachable!(),
        };

        match self.piece_type {
            PieceType::I => &I_KICKS[kick_index],
            PieceType::O => &[(0, 0)],
            _ => &DEFAULT_KICKS[kick_index],
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.board.cols;
        for (i, j) in self.all_cells() {
            let ch = match self.board.get(i, j) {
                Cell::Filled(_) => '*',
                Cell::Empty => ' ',
            };
            write!(f, "{}", ch)?;
            if j == c - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn fill_piece_matrix(piece: PieceType, matrix: &mut Board, rotation: Rotation) {
    let matrix_bytes = get_rotations(piece, rotation);
    let cols = matrix.cols;

    for (row, row_vec) in matrix_bytes.iter().rev().zip(matrix.rows_mut()) {
        let mut acc = 1u8 << (cols - 1);
        for col in row_vec {
            let ch = match row & acc {
                0 => None,
                _ => Some(piece),
            };

            *col = ch.map(playable_piece_to_cell).unwrap_or(Cell::Empty);

            acc >>= 1;
        }
    }
}

fn get_piece_matrix(piece: PieceType, rotation: Rotation) -> Board {
    let (r, c) = get_piece_size(piece);

    let mut matrix = Board::new(r, c);

    fill_piece_matrix(piece, &mut matrix, rotation);

    matrix
}

fn get_piece_size(piece: PieceType) -> (isize, isize) {
    match piece {
        PieceType::I => (4, 4),
        PieceType::O => (3, 4),
        _ => (3, 3),
    }
}

pub struct PieceWithPosition {
    r: isize,
    c: isize,
    piece: Piece,
}

impl PieceWithPosition {
    pub fn new(r: isize, c: isize, piece: Piece) -> Self {
        PieceWithPosition { r, c, piece }
    }

    pub fn row(&self) -> isize {
        self.r
    }

    pub fn col(&self) -> isize {
        self.c
    }

    pub fn tetris_piece(self) -> Piece {
        self.piece
    }

    pub fn tetris_piece_ref(&self) -> &Piece {
        &self.piece
    }

    pub fn tetris_piece_mut(&mut self) -> &mut Piece {
        &mut self.piece
    }

    pub fn finalize_on(&self, board: &mut Board) {
        for (i, j) in self.piece.set_cells() {
            board.set(
                i + self.row(),
                j + self.col(),
                PieceTypeColor::Playable(self.piece.piece_type),
            );
        }
    }

    pub fn collides_left(&self, matrix: &Board) -> bool {
        self.piece.collides(self.r, self.c - 1, matrix)
    }

    pub fn collides_right(&self, matrix: &Board) -> bool {
        self.piece.collides(self.r, self.c + 1, matrix)
    }

    pub fn collides_down(&self, matrix: &Board) -> bool {
        self.piece.collides(self.r - 1, self.c, matrix)
    }

    pub fn collides_kick(&self, matrix: &Board, (k0, k1): &Kick) -> bool {
        self.piece.collides(self.r + k1, self.c + k0, matrix)
    }

    pub fn try_move_left(&mut self, matrix: &Board) -> bool {
        if !self.collides_left(matrix) {
            self.move_left();
            true
        } else {
            false
        }
    }

    pub fn try_move_right(&mut self, matrix: &Board) -> bool {
        if !self.collides_right(matrix) {
            self.move_right();
            true
        } else {
            false
        }
    }

    pub fn kick_by(&mut self, kick: &Kick) {
        self.r -= kick.1;
        self.c += kick.0;
    }

    pub fn rotate_and_kick(&self, prev_rot: Rotation, matrix: &Board) -> Option<&'static Kick> {
        self.piece
            .get_kicks(prev_rot)
            .iter()
            .find(|&kick| !self.collides_kick(matrix, kick))
    }

    pub fn move_down(&mut self) {
        self.r -= 1;
    }

    pub fn move_left(&mut self) {
        self.c -= 1;
    }

    pub fn move_right(&mut self) {
        self.c += 1;
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{get_piece_size, Board, Piece, PieceType, Rotation};
    use crate::test_utils::load_board;
    use test_case::test_case;

    #[test_case(PieceType::T, 3, 3; "T")]
    #[test_case(PieceType::I, 4, 4; "I")]
    #[test_case(PieceType::S, 3, 3; "S")]
    #[test_case(PieceType::Z, 3, 3; "Z")]
    #[test_case(PieceType::J, 3, 3; "J")]
    #[test_case(PieceType::L, 3, 3; "L")]
    #[test_case(PieceType::O, 3, 4; "O")]
    fn test_get_piece_size(piece_type: PieceType, exp_w: isize, exp_h: isize) {
        let (w, h) = get_piece_size(piece_type);
        assert!(w == exp_w);
        assert!(h == exp_h);
    }

    #[test]
    fn test_next_rotation() {
        let mut rotation = Rotation::R0;
        let rotations = vec![
            Rotation::R0,
            Rotation::R90,
            Rotation::R180,
            Rotation::R270,
            Rotation::R0,
        ];

        for r in rotations {
            assert_eq!(rotation, r);
            rotation = rotation.next();
        }
    }

    #[test]
    fn test_prev_rotation() {
        let mut rotation = Rotation::R0;
        let rotations = vec![
            Rotation::R0,
            Rotation::R270,
            Rotation::R180,
            Rotation::R90,
            Rotation::R0,
        ];

        for r in rotations {
            assert_eq!(rotation, r);
            rotation = rotation.prev();
        }
    }

    #[test]
    fn test_set_cells_T() {
        let t = Piece::new(PieceType::T);
        let cells: Vec<_> = t.set_cells().collect();

        assert_eq!(cells, vec![(0, 1), (1, 0), (1, 1), (1, 2)]);
    }

    #[test]
    fn test_set_cells_I() {
        let t = Piece::new(PieceType::I);
        let cells: Vec<_> = t.set_cells().collect();

        assert_eq!(cells, vec![(1, 0), (1, 1), (1, 2), (1, 3)]);
    }

    #[test]
    fn test_collides_right_true() {
        let mut b = Board::new(4, 5);
        load_board(&mut b, "        **    ******");
        let mut t = Piece::new(PieceType::T);
        for _ in 1..=3 {
            assert!(t.collides(0, 1, &b), "current rotation: {:?}", t.rotation);
            t.rotate_piece();
        }
        assert!(!t.collides(0, 1, &b), "current rotation: {:?}", t.rotation);
    }

    #[test]
    fn test_collides_right_wall_true() {
        let mut b = Board::new(3, 3);
        let s = " ".repeat(9);
        load_board(&mut b, &s);
        let mut t = Piece::new(PieceType::T);
        for _ in 1..=3 {
            assert!(t.collides(0, 1, &b), "current rotation: {:?}", t.rotation);
            t.rotate_piece();
        }
        assert!(!t.collides(0, 1, &b), "current rotation: {:?}", t.rotation);
    }

    #[test]
    fn test_collides_right_false() {
        let mut b = Board::new(4, 5);
        load_board(&mut b, "         *    ******");
        let mut t = Piece::new(PieceType::T);
        for _ in 1..=4 {
            assert!(!t.collides(0, 1, &b), "current rotation: {:?}", t.rotation);
            t.rotate_piece();
        }
    }

    #[test]
    fn test_display_t() {
        let t = Piece::new(PieceType::T);
        let s = format!("{}", t);
        assert_eq!(s, " * \n***\n   \n");

        let t = Piece::with_rotation(PieceType::T, Rotation::R180);
        let s = format!("{}", t);
        assert_eq!(s, "   \n***\n * \n");
    }
}
