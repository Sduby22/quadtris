use std::mem;

use log::{debug, trace};

use crate::{
    board::Board,
    constants::{C, R},
    enums::PieceType,
    pieces::PieceWithPosition,
};

#[derive(Debug, Clone, Copy)]
pub enum ScoreType {
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    TSpinMini,
    Tetris,
    AllClear,
    Single,
    Double,
    Triple,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Moves {
    Fall,
    Rotate,
    Side,
    Down,
    Up,
}

pub struct ScoreUpdater {
    current_combo: ComboState,
    last_score: Option<ScoreType>,
    last_move: Moves,
    back_to_back: u32,
}

impl ScoreUpdater {
    pub fn set_last_move(&mut self, last_move: Moves) {
        self.last_move = last_move;
    }

    pub fn get_last_score(&self) -> Option<ScoreType> {
        self.last_score
    }

    pub fn get_b2b(&self) -> u32 {
        self.back_to_back
    }

    pub fn get_combo(&self) -> ComboState {
        self.current_combo
    }

    pub fn finalized_piece(&mut self, piece_with_position: &PieceWithPosition, board: &mut Board) {
        let completed_rows_ranges = board.completed_rows();
        let completed_rows = completed_rows_ranges
            .iter()
            .map(|r| (r.0 - r.1) as u32)
            .sum();

        let cur = mem::take(&mut self.current_combo);
        self.current_combo = cur.next(completed_rows);

        let last = self.last_score.take();

        if piece_with_position.tetris_piece_ref().piece_type == PieceType::T {
            // detect T-spin

            if completed_rows > 0 && self.last_move == Moves::Rotate {
                let center_r = piece_with_position.row() + 1;
                let center_c = piece_with_position.col() + 1;
                let mut occupied = 0;

                debug!("Maybe t-spin detected");

                for i in &[-1, 1] {
                    for j in &[-1, 1] {
                        let ei = center_r + i;
                        let ej = center_c + j;

                        if !(0..R).contains(&ei) || !(0..C).contains(&ej) || board.is_set(ei, ej) {
                            occupied += 1;
                        }
                    }
                }

                debug!("{} corners occupied", occupied);

                if occupied >= 3 {
                    self.last_score = match completed_rows {
                        1 => Some(ScoreType::TSpinSingle),
                        2 => Some(ScoreType::TSpinDouble),
                        3 => Some(ScoreType::TSpinTriple),
                        _ => None,
                    };

                    debug!("Score computed: {:?}", self.last_score);
                }
            }
        } else if (2..=4).contains(&completed_rows) {
            self.last_score = match completed_rows {
                2 => Some(ScoreType::Double),
                3 => Some(ScoreType::Triple),
                4 => {
                    debug!("Tetris detected");
                    Some(ScoreType::Tetris)
                }
                _ => unreachable!(),
            }
        }

        let is_b2b = completed_rows > 0
            && last.is_some()
            && self.last_score.is_some()
            && is_b2b_worth(self.last_score.unwrap());
        trace!("B2B detected? {}", is_b2b);

        if is_b2b {
            self.back_to_back += 1;
        } else {
            self.back_to_back = 0;
        }

        if self.back_to_back > 0 {
            debug!("B2B level: {}", self.back_to_back);
        }

        board.remove_ranges(completed_rows_ranges);

        if board.is_empty() {
            self.back_to_back = 0;
            self.last_score = Some(ScoreType::AllClear);
        }
    }
}

impl Default for ScoreUpdater {
    fn default() -> Self {
        ScoreUpdater {
            current_combo: ComboState::Empty,
            last_move: Moves::Fall,
            last_score: None,
            back_to_back: 0,
        }
    }
}

pub fn is_b2b_worth(s: ScoreType) -> bool {
    use ScoreType::*;
    !matches!(s, Single | Double | Triple)
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum ComboState {
    #[default]
    Empty,
    Started,
    Continuing(u32),
}

impl ComboState {
    pub fn next(self, completed_rows: u32) -> ComboState {
        match completed_rows {
            0 => ComboState::Empty,
            _ => match self {
                ComboState::Empty => ComboState::Started,
                ComboState::Started => ComboState::Continuing(1),
                ComboState::Continuing(n) => ComboState::Continuing(n + 1),
            },
        }
    }

    pub fn value(&self) -> Option<u32> {
        if let ComboState::Continuing(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComboState::*;
    use super::ScoreType::*;
    use super::*;

    #[test]
    fn test_combo_next() {
        let rows = [1, 2, 1, 2, 0, 1, 1, 2, 0, 0, 1, 0];
        let states = [
            Started,
            Continuing(1),
            Continuing(2),
            Continuing(3),
            Empty,
            Started,
            Continuing(1),
            Continuing(2),
            Empty,
            Empty,
            Started,
            Empty,
        ];
        let mut cur = Empty;

        for (r, s) in rows.into_iter().zip(states.into_iter()) {
            cur = cur.next(r);
            assert_eq!(cur, s);
        }
    }

    #[test]
    fn test_combo_value() {
        assert_eq!(Empty.value(), None);
        assert_eq!(Started.value(), None);
        assert_eq!(Continuing(1).value(), Some(1));
        assert_eq!(Continuing(5).value(), Some(5));
    }

    #[test]
    fn test_is_b2b_worth() {
        assert!(is_b2b_worth(TSpinSingle));
        assert!(is_b2b_worth(TSpinDouble));
        assert!(is_b2b_worth(TSpinTriple));
        assert!(is_b2b_worth(TSpinMini));
        assert!(is_b2b_worth(Tetris));
        assert!(is_b2b_worth(AllClear));
        assert!(!is_b2b_worth(Single));
        assert!(!is_b2b_worth(Double));
        assert!(!is_b2b_worth(Triple));
    }
}
