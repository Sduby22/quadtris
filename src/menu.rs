use macroquad::prelude::*;

use crate::{
    constants::FONT_SIZE,
    game_data::Key,
    renderer::text::{self, TextRenderer},
};

pub struct MenuCtx {
    pub states: Vec<MenuState>,
    pub curr_pointer: i32,
    pub modifying: bool,
}

impl MenuCtx {
    pub fn new() -> Self {
        Self {
            states: vec![MenuState::Main],
            curr_pointer: 0,
            modifying: false,
        }
    }

    pub fn curr_state(&self) -> MenuState {
        *self.states.last().unwrap()
    }

    pub fn pop_state(&mut self) {
        if self.states.len() > 1 {
            self.states.pop();
            self.curr_pointer = 0;
        }
    }

    pub fn push_state(&mut self, state: MenuState) {
        if state != self.curr_state() {
            self.states.push(state);
            self.curr_pointer = 0;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    Main,
    Settings,
}

pub struct Menu<'a> {
    widgets: Vec<Box<dyn 'a + MenuWidget>>,
    ctx: &'a mut MenuCtx,
    text_renderer: &'a TextRenderer,
}

impl<'a> Menu<'a> {
    pub fn new(ctx: &'a mut MenuCtx, text_renderer: &'a TextRenderer) -> Self {
        Self {
            ctx,
            text_renderer,
            widgets: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.widgets.len()
    }

    pub fn add_widget(&mut self, mut widget: impl 'a + MenuWidget) {
        widget.insert_menu(self.widgets.len() as i32);
        self.widgets.push(Box::new(widget));
    }

    pub fn draw(&mut self, mut position: Vec2) {
        for widget in self.widgets.iter_mut() {
            widget.draw(position, self.text_renderer, self.ctx);
            position.y -= widget.get_height();
            widget.handle_input(self.ctx);
        }
    }
}

pub trait MenuWidget {
    fn draw(&self, position: Vec2, text_renderer: &TextRenderer, ctx: &MenuCtx);
    fn handle_input(&mut self, ctx: &mut MenuCtx);
    fn get_height(&self) -> f32;
    fn insert_menu(&mut self, id: i32);
}

pub struct Button<'a> {
    label: &'a str,
    callback: Box<dyn 'a + FnMut()>,
    id: i32,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str, callback: impl 'a + FnMut()) -> Self {
        Self {
            label,
            callback: Box::new(callback),
            id: -1,
        }
    }
}

impl<'a> MenuWidget for Button<'a> {
    fn draw(&self, position: Vec2, text_renderer: &TextRenderer, ctx: &MenuCtx) {
        text_renderer.draw_text(
            self.label,
            position,
            FONT_SIZE,
            if self.id == ctx.curr_pointer {
                text::Color::Cream
            } else {
                text::Color::Magenta
            },
        );
    }

    fn handle_input(&mut self, ctx: &mut MenuCtx) {
        if self.id == ctx.curr_pointer && is_key_pressed(KeyCode::Enter) {
            (self.callback)();
        }
    }

    fn get_height(&self) -> f32 {
        FONT_SIZE
    }

    fn insert_menu(&mut self, id: i32) {
        self.id = id;
    }
}

pub struct Selector<'a, T> {
    label: &'a str,
    value: &'a mut T,
    optional_value: &'a [T],
    optional_value_labels: &'a [String],
    id: i32,
}

impl<'a, T: PartialEq + Clone> Selector<'a, T> {
    pub fn new(
        label: &'a str,
        value: &'a mut T,
        optional_value: &'a [T],
        optional_value_labels: &'a [String],
    ) -> Self {
        Self {
            label,
            value,
            optional_value,
            optional_value_labels,
            id: -1,
        }
    }

    fn curr_index(&self) -> usize {
        self.optional_value
            .iter()
            .position(|v| *v == *self.value)
            .unwrap_or(0)
    }

    fn select_left(&mut self) {
        let curr_index = self.curr_index();
        if curr_index > 0 {
            *self.value = self.optional_value[curr_index - 1].clone();
        }
    }

    fn select_right(&mut self) {
        let curr_index = self.curr_index();
        if curr_index < self.optional_value.len() - 1 {
            *self.value = self.optional_value[curr_index + 1].clone();
        }
    }
}

impl<'a, T: PartialEq + Clone> MenuWidget for Selector<'a, T> {
    fn draw(&self, position: Vec2, text_renderer: &TextRenderer, ctx: &MenuCtx) {
        let text = format!(
            "{:<9}<{:^5}>",
            self.label,
            self.optional_value_labels[self.curr_index()]
        );

        text_renderer.draw_text(
            &text,
            position,
            FONT_SIZE,
            if self.id == ctx.curr_pointer {
                text::Color::Cream
            } else {
                text::Color::Magenta
            },
        );
    }

    fn handle_input(&mut self, ctx: &mut MenuCtx) {
        if self.id == ctx.curr_pointer {
            if is_key_pressed(KeyCode::Left) {
                self.select_left();
            } else if is_key_pressed(KeyCode::Right) {
                self.select_right();
            }
        }
    }

    fn get_height(&self) -> f32 {
        FONT_SIZE
    }

    fn insert_menu(&mut self, id: i32) {
        self.id = id;
    }
}

pub struct KeyBind<'a> {
    value: &'a mut Key,
    label: &'a str,
    id: i32,
}

impl<'a> KeyBind<'a> {
    pub fn new(value: &'a mut Key, label: &'a str) -> Self {
        Self {
            value,
            label,
            id: -1,
        }
    }
}

impl<'a> MenuWidget for KeyBind<'a> {
    fn draw(&self, position: Vec2, text_renderer: &TextRenderer, ctx: &MenuCtx) {
        let key = if ctx.modifying && self.id == ctx.curr_pointer {
            "WAIT".to_string()
        } else {
            format!("{:?}", self.value.code())
        };

        let text = format!("{:<11}{:>5}", self.label, key);

        text_renderer.draw_text(
            &text,
            position,
            FONT_SIZE,
            if self.id == ctx.curr_pointer {
                text::Color::Cream
            } else {
                text::Color::Magenta
            },
        );
    }

    fn handle_input(&mut self, ctx: &mut MenuCtx) {
        if self.id != ctx.curr_pointer {
            return;
        }

        if ctx.modifying {
            if let Some(keycode) = get_last_key_pressed() {
                match keycode {
                    KeyCode::Escape | KeyCode::Enter => {}
                    _ => {
                        *self.value = Key::new(keycode);
                    }
                }
            }
        } else if is_key_pressed(KeyCode::Enter) {
            ctx.modifying = true;
        }
    }

    fn get_height(&self) -> f32 {
        FONT_SIZE
    }

    fn insert_menu(&mut self, id: i32) {
        self.id = id;
    }
}

pub struct Margin;

impl MenuWidget for Margin {
    fn draw(&self, _: Vec2, _: &TextRenderer, _: &MenuCtx) {}

    fn handle_input(&mut self, _: &mut MenuCtx) {}

    fn get_height(&self) -> f32 {
        FONT_SIZE
    }

    fn insert_menu(&mut self, _: i32) {}
}
