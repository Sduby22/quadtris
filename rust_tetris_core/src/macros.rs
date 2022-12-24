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
