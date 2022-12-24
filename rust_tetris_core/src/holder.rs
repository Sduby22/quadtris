use crate::{enums::Rotation, pieces::Piece};

pub struct HoldPiece {
    pub piece: Piece,
    pub already_hold: bool,
}

impl HoldPiece {
    pub fn new(mut piece: Piece) -> Self {
        piece.set_rotation(Rotation::R0);
        HoldPiece {
            piece,
            already_hold: true,
        }
    }

    pub fn set_hold(&mut self) {
        self.already_hold = true;
    }

    pub fn reset_hold(&mut self) {
        self.already_hold = false;
    }
}

pub trait Swappable {
    fn can_swap(&self) -> bool;
}

impl Swappable for Option<HoldPiece> {
    fn can_swap(&self) -> bool {
        self.as_ref().map(|h| !h.already_hold).unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::PieceType::*;

    #[test]
    fn test_swappable_option() {
        let empty: Option<HoldPiece> = None;
        assert!(empty.can_swap());

        let mut h = HoldPiece::new(Piece::new(I));
        h.reset_hold();
        let not_already_hold: Option<HoldPiece> = Some(h);
        assert!(not_already_hold.can_swap());

        let mut h = HoldPiece::new(Piece::new(I));
        h.set_hold();
        let already_hold: Option<HoldPiece> = Some(h);
        assert!(!already_hold.can_swap());
    }

    #[test]
    fn test_new() {
        let mut p = Piece::new(I);
        p.rotate_piece();
        assert_ne!(p.rotation, Rotation::R0);
        let h = HoldPiece::new(p);
        assert_eq!(h.piece.rotation, Rotation::R0);
        assert!(h.already_hold);
    }

    #[test]
    fn test_set_clear() {
        let p = Piece::new(I);
        let mut h = HoldPiece::new(p);

        assert!(h.already_hold);
        h.reset_hold();
        assert!(!h.already_hold);
        h.set_hold();
        assert!(h.already_hold);
        h.set_hold();
        assert!(h.already_hold);
        h.reset_hold();
        assert!(!h.already_hold);
        h.reset_hold();
        assert!(!h.already_hold);
        h.set_hold();
        assert!(h.already_hold);
    }
}
