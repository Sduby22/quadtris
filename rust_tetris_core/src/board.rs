use crate::enums::{PieceType, PieceTypeColor};
use std::fmt::{Debug, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Filled(PieceTypeColor),
    Empty,
}

pub fn is_filled(cell: Cell) -> bool {
    match cell {
        Cell::Filled(_) => true,
        Cell::Empty => false,
    }
}

pub fn playable_piece_to_cell(piece: PieceType) -> Cell {
    Cell::Filled(PieceTypeColor::Playable(piece))
}

pub fn not_playable_piece_to_cell() -> Cell {
    Cell::Filled(PieceTypeColor::NotPlayable)
}

#[derive(Clone)]
pub struct Board {
    pub rows: isize,
    pub cols: isize,
    data: Vec<Vec<Cell>>,
    empty_row_proto: Vec<Cell>,
}

impl Board {
    pub fn new(rows: isize, cols: isize) -> Self {
        let mut board = Board {
            rows,
            cols,
            data: Vec::with_capacity(rows as usize),
            empty_row_proto: vec![Cell::Empty; cols as usize],
        };

        for _ in 0..rows {
            board.data.push(board.empty_row_proto.clone());
        }

        board
    }

    pub fn get(&self, i: isize, j: isize) -> Cell {
        self.data[i as usize][j as usize]
    }

    pub fn is_in_bounds(&self, i: isize, j: isize) -> bool {
        !(i >= self.rows || j >= self.cols || i < 0 || j < 0)
    }

    pub fn is_set(&self, i: isize, j: isize) -> bool {
        if !self.is_in_bounds(i, j) {
            false
        } else {
            is_filled(self.data[i as usize][j as usize])
        }
    }

    pub fn set(&mut self, i: isize, j: isize, p: PieceTypeColor) {
        self.set_val(i, j, Cell::Filled(p));
    }

    pub fn clear(&mut self, i: isize, j: isize) {
        self.set_val(i, j, Cell::Empty);
    }

    pub fn set_val(&mut self, i: isize, j: isize, b: Cell) {
        self.data[i as usize][j as usize] = b;
    }

    pub fn is_complete(&self, i: isize) -> bool {
        self.data[i as usize].iter().all(|&cell| is_filled(cell))
    }

    pub fn is_empty(&self) -> bool {
        self.data
            .iter()
            .all(|row| row.iter().all(|&cell| !is_filled(cell)))
    }

    pub fn completed_rows(&mut self) -> Vec<(isize, isize)> {
        let mut ranges = vec![];

        let mut from = None;
        let mut to: Option<isize> = None;

        for i in (0..self.data.len()).rev() {
            let i = i as isize;

            if self.is_complete(i) {
                if from.is_none() {
                    from = Some(i);
                } else {
                    to = Some(i);
                }
            } else if from.is_some() {
                let from_i = from.unwrap();
                let to_i = to.unwrap_or(from_i);

                ranges.push((from_i, to_i - 1));
                from = None;
                to = None;
            }
        }

        if let Some(from_i) = from {
            let to_i = to.unwrap_or(from_i);

            ranges.push((from_i, to_i - 1));
        }

        ranges
    }

    pub fn remove_ranges(&mut self, ranges: Vec<(isize, isize)>) {
        for range in &ranges {
            self.remove_rows(range.0, range.1);
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.data.iter()
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<Cell>> {
        self.data.iter_mut()
    }

    pub fn remove_row(&mut self, row: isize) {
        self.remove_rows(row, row - 1)
    }

    pub fn remove_rows(&mut self, from: isize, to: isize) {
        if from == to {
            return;
        }

        let offset = from - to;
        let move_down_begin = from + 1;
        let move_down_end = self.rows;

        for i in to + 1..=from {
            self.data[i as usize] = self.empty_row_proto.clone();
        }

        for i in move_down_begin..move_down_end {
            self.data.swap(i as usize, (i - offset) as usize);
        }
    }

    pub fn get_first_set_col(&self) -> Option<isize> {
        (0..self.cols)
            .flat_map(|j| (0..self.rows).map(move |i| (i, j)))
            .find(|&(i, j)| self.is_set(i, j))
            .map(|(_, j)| j)
    }

    pub fn get_last_set_col(&self) -> Option<isize> {
        (0..self.cols)
            .rev()
            .flat_map(|j| (0..self.rows).map(move |i| (i, j)))
            .find(|&(i, j)| self.is_set(i, j))
            .map(|(_, j)| j)
    }
}

impl Debug for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        for i in (0..self.rows).rev() {
            for j in 0..self.cols {
                let c = match self.get(i, j) {
                    Cell::Filled(_) => '*',
                    Cell::Empty => ' ',
                };
                write!(formatter, "{}", c)?
            }

            writeln!(formatter)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::load_board;
    use test_case::test_case;

    #[test_case(Cell::Filled(PieceTypeColor::NotPlayable), true; "when cell is filled with not playable")]
    #[test_case(Cell::Filled(PieceTypeColor::Playable(PieceType::I)), true; "when cell is filled with playable")]
    #[test_case(Cell::Empty, false; "when cell is empty")]
    fn test_is_filled(cell: Cell, expected: bool) {
        assert_eq!(is_filled(cell), expected)
    }

    #[test]
    fn test_playable_piece_to_cell() {
        let pt = PieceType::I;
        assert_eq!(
            playable_piece_to_cell(pt),
            Cell::Filled(PieceTypeColor::Playable(pt))
        );
    }

    #[test]
    fn test_not_playable_piece_to_cell() {
        assert_eq!(
            not_playable_piece_to_cell(),
            Cell::Filled(PieceTypeColor::NotPlayable)
        );
    }

    #[test]
    fn test_board_is_in_bounds() {
        let mut board = Board::new(5, 3);
        load_board(&mut board, "     *  *******");

        assert!(!board.is_in_bounds(-1, 0));
        assert!(!board.is_in_bounds(0, -1));
        assert!(board.is_in_bounds(0, 1));
        assert!(board.is_in_bounds(0, 2));
        assert!(!board.is_in_bounds(6, 0));
        assert!(!board.is_in_bounds(0, 4));
    }

    #[test_case(0, 0, Cell::Empty; "when cell is empty")]
    #[test_case(1, 2, Cell::Filled(PieceTypeColor::NotPlayable); "when cell is not playable")]
    #[test_case(2, 2, Cell::Filled(PieceTypeColor::Playable(PieceType::O)); "when cell is O")]
    #[test_case(3, 0, Cell::Filled(PieceTypeColor::Playable(PieceType::I)); "when cell is I")]
    #[test_case(3, 1, Cell::Filled(PieceTypeColor::Playable(PieceType::L)); "when cell is L")]
    #[test_case(3, 2, Cell::Filled(PieceTypeColor::Playable(PieceType::T)); "when cell is T")]
    #[test_case(4, 0, Cell::Filled(PieceTypeColor::Playable(PieceType::S)); "when cell is S")]
    #[test_case(4, 1, Cell::Filled(PieceTypeColor::Playable(PieceType::J)); "when cell is J")]
    #[test_case(4, 2, Cell::Filled(PieceTypeColor::Playable(PieceType::Z)); "when cell is Z")]
    fn test_board_get(r: isize, c: isize, res: Cell) {
        let mut board = Board::new(5, 3);
        load_board(&mut board, "     *  OILTSJZ");
        assert_eq!(board.get(r, c), res);
    }

    #[test_case(0, 0, false; "when cell is empty")]
    #[test_case(1, 2, true; "when cell is not playable")]
    #[test_case(2, 2, true; "when cell is playable")]
    #[test_case(-2, 2, false; "when cell is out of bounds")]
    fn test_is_set(r: isize, c: isize, res: bool) {
        let mut board = Board::new(5, 3);
        load_board(&mut board, "     *  OILTSJZ");
        assert_eq!(board.is_set(r, c), res);
    }

    #[test]
    fn test_board_remove_rows() {
        let mut board = Board::new(5, 3);

        load_board(&mut board, "*** * *********");

        let ranges = board.completed_rows();
        board.remove_ranges(ranges);

        for i in 0..5 {
            for j in 0..3 {
                if i == 0 && j == 1 {
                    assert!(is_filled(board.get(i, j)));
                } else {
                    assert!(!is_filled(board.get(i, j)));
                }
            }
        }
    }

    #[test]
    fn test_board_remove_rows3() {
        let mut board = Board::new(5, 3);

        load_board(&mut board, "      *********");

        let ranges = board.completed_rows();
        board.remove_ranges(ranges);

        for i in 0..5 {
            for j in 0..3 {
                assert!(!is_filled(board.get(i, j)));
            }
        }
    }
}
