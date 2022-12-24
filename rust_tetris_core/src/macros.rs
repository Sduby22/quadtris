macro_rules! rotations {
    ($rot:expr, Z => $z:expr, R => $r:expr, T => $t:expr, L => $l:expr) => {{
        use Rotation::*;
        let r = match $rot {
            R0 => $z,
            R90 => $r,
            R180 => $t,
            R270 => $l,
        };
        Vec::from(r)
    }};
}

#[macro_export]
macro_rules! piece {
    (
        $piece:expr,
        O => $o: expr,
        I => $i: expr,
        Z => $z: expr,
        S => $s: expr,
        J => $j: expr,
        L => $l: expr,
        T => $t: expr,
    ) => {{
        use PieceType::*;
        match $piece {
            O => $o,
            I => $i,
            Z => $z,
            S => $s,
            J => $j,
            L => $l,
            T => $t,
        }
    }};
}
