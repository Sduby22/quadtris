use macroquad::prelude::*;

pub fn push_model_matrix(mat: Mat4) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(mat);
    };
}

pub fn pop_model_matrix() {
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    };
}
