use lazy_static::lazy_static;
use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

pub fn use_custom_material() {
    gl_use_material(*MAT);
}

lazy_static! {
    static ref MAT: Material = load_material(
        VERTEX,
        FRAGMENT,
        MaterialParams {
            pipeline_params: PipelineParams {
                cull_face: miniquad::CullFace::Back,
                depth_write: true,
                depth_test: Comparison::Less,
                primitive_type: miniquad::PrimitiveType::Triangles,
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();
}

pub const VERTEX: &str = r#"#version 100
    attribute vec3 position;
    attribute vec2 texcoord;

    varying lowp vec2 uv;
    varying lowp vec4 color;

    uniform mat4 Model;
    uniform mat4 Projection;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = vec4(1,1,1,0.4);
        uv = texcoord;
    }"#;

pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 uv;

    uniform sampler2D Texture;

    void main() {
        gl_FragColor = color * texture2D(Texture, uv) ;
    }"#;
