use std::collections::VecDeque;

use crate::{enums::PieceType, pieces::Piece};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

/// Struct implementing a tetris bag.
pub struct PieceBag {
    /// Buffer holding the next pieces, from back to front
    buffer_next_pieces: VecDeque<Piece>,
    /// The internal permutation
    internal_permutation: Vec<PieceType>,
    /// the random generator used for generating the bags
    rng: StdRng,
    /// the buffer size, usually 5 in standard tetris
    buffer_size: usize,
}

impl PieceBag {
    pub fn new(buffer_size: usize, seed: Option<u64>) -> Self {
        let mut inst = PieceBag {
            buffer_next_pieces: VecDeque::with_capacity(buffer_size),
            internal_permutation: Vec::with_capacity(7),
            rng: seed.map_or_else(StdRng::from_entropy, StdRng::seed_from_u64),
            buffer_size,
        };
        inst.fill_permutation();

        for _ in 0..inst.buffer_size {
            inst.new_block_in_buffer();
        }

        inst
    }

    /// Returns the next piece in the bag.
    /// If the buffer is empty, it gets filled from the permutation.
    pub fn next_piece(&mut self) -> Piece {
        let ret = self.buffer_next_pieces.pop_back().unwrap();
        self.new_block_in_buffer();
        ret
    }

    /// Adds a piece to the start of the buffer.
    /// Typically used only for the debug commands.
    pub fn add_top(&mut self, p: Piece) {
        self.buffer_next_pieces.push_back(p);
    }

    /// Returns an iterator over the next buffer.
    pub fn iter_next_pieces(&self) -> impl Iterator<Item = &Piece> {
        self.buffer_next_pieces.iter().rev().take(self.buffer_size)
    }

    fn fill_permutation(&mut self) {
        let mut nums: Vec<_> = vec![
            PieceType::I,
            PieceType::S,
            PieceType::Z,
            PieceType::O,
            PieceType::T,
            PieceType::L,
            PieceType::J,
        ];
        nums.as_mut_slice().shuffle(&mut self.rng);
        self.internal_permutation.extend(nums.into_iter());
    }

    fn new_block_in_buffer(&mut self) {
        if self.internal_permutation.is_empty() {
            self.fill_permutation();
        }

        let piece = self.internal_permutation.pop().unwrap();
        self.buffer_next_pieces.push_front(Piece::new(piece));
    }
}

impl Default for PieceBag {
    /// Creates a default `PieceBag`.
    fn default() -> Self {
        PieceBag::new(5, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PieceType::*;

    const FIRST_ORDER: [PieceType; 7] = [J, S, Z, I, T, O, L];
    const SECOND_ORDER: [PieceType; 7] = [L, T, Z, I, O, S, J];

    fn get_bag() -> PieceBag {
        // this seed leads to the orders defined above
        PieceBag::new(5, Some(0xDEADBEEF))
    }

    #[test]
    fn test_init() {
        let bag = get_bag();
        assert_eq!(bag.buffer_size, 5);

        let (exp_perm, exp_buf) = FIRST_ORDER.split_at(2);
        assert_eq!(bag.internal_permutation, exp_perm);
        assert_eq!(
            bag.buffer_next_pieces
                .iter()
                .map(|p| p.piece_type)
                .collect::<Vec<_>>(),
            exp_buf
        );
    }

    #[test]
    fn test_next() {
        let mut bag = get_bag();

        assert_eq!(
            (0..14)
                .map(|_| bag.next_piece().piece_type)
                .collect::<Vec<_>>(),
            FIRST_ORDER
                .iter()
                .rev()
                .chain(SECOND_ORDER.iter().rev())
                .copied()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_add_top() {
        let mut bag = get_bag();
        let added = vec![J, I, O];

        for p in &added {
            bag.add_top(Piece::new(*p));
        }

        let res: Vec<_> = (0..10).map(|_| bag.next_piece().piece_type).collect();
        assert_eq!(
            res,
            added
                .into_iter()
                .rev()
                .chain(FIRST_ORDER.iter().rev().copied())
                .collect::<Vec<_>>()
        );
    }
}
