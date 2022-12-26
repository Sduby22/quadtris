use macroquad::prelude::*;

pub struct TextureAltas {
    pub image: Image,
    pub altas: Vec<Texture2D>,
    pub rows: usize,
    pub cols: usize,
    pub unit_width: usize,
    pub unit_height: usize,
    pub padding_row: usize,
    pub padding_col: usize,
}

impl TextureAltas {
    pub fn new(
        image: Image,
        rows: usize,
        cols: usize,
        unit_width: usize,
        unit_height: usize,
        padding_row: usize,
        padding_col: usize,
    ) -> Self {
        Self {
            altas: Self::build_atlas(
                &image,
                rows,
                cols,
                unit_width,
                unit_height,
                padding_row,
                padding_col,
            ),
            image,
            rows,
            cols,
            unit_height,
            unit_width,
            padding_row,
            padding_col,
        }
    }

    pub fn get_texture(&self, index: usize) -> Texture2D {
        self.altas[index]
    }

    fn build_atlas(
        texture: &Image,
        rows: usize,
        cols: usize,
        unit_width: usize,
        unit_height: usize,
        padding_row: usize,
        padding_col: usize,
    ) -> Vec<Texture2D> {
        let mut res = vec![];

        for r in 0..rows {
            for c in 0..cols {
                let x = c * (unit_width + padding_col);
                let y = r * (unit_height + padding_row);
                let img = texture.sub_image(Rect::new(
                    x as f32,
                    y as f32,
                    unit_width as f32,
                    unit_height as f32,
                ));
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                res.push(t);
            }
        }

        res
    }
}
