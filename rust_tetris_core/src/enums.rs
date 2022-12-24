custom_derive! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumFromStr)]
pub enum PieceType {
    T,
    L,
    J,
    O,
    I,
    S,
    Z,
}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceTypeColor {
    Playable(PieceType),
    NotPlayable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl From<u8> for Rotation {
    fn from(u: u8) -> Self {
        match u {
            0 => Rotation::R0,
            1 => Rotation::R90,
            2 => Rotation::R180,
            3 => Rotation::R270,
            _ => panic!("Invalid rotation"),
        }
    }
}

impl Rotation {
    pub fn next(self) -> Self {
        Rotation::from((self as u8 + 1) % 4)
    }

    pub fn prev(self) -> Self {
        Rotation::from((self as u8 + 3) % 4)
    }
}
