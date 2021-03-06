use crate::commands::{Color, Vec2f};

#[derive(Clone, Copy, Default, Debug)]
pub struct GridComponent {
    pub step: i32,
    pub color: Color,
}

#[derive(Default, Debug)]
pub struct ViewPortSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Default, Debug)]
pub struct WorkAreaComponent {
    pub title: String,
    pub color: Color,
    pub size: Vec2f,
}

#[derive(Default, Debug)]
pub struct Camera2D {
    pub id: usize,
    pub pos: Vec2f,
}

#[derive(Default, Debug)]
pub struct CameraMovable2D {
    pub is_hot: bool,
    pub last_pos: Vec2f,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera2DPositionListener {
    pub id: usize,
    pub pos: Vec2f,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actions;

impl Camera2DPositionListener {
    pub fn new(id: usize) -> Camera2DPositionListener {
        Camera2DPositionListener {
            id,
            pos: Vec2f::ZERO,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Touch {
    None,
    Start,
    Move,
    End,
}

#[derive(Debug)]
pub struct TouchState {
    pub last_touch: Touch,
    pub touch: Touch,
    pub pos: Vec2f,
    pub touch_start: Vec2f,
    pub touch_current: Vec2f,
}

#[derive(Debug, Default)]
pub struct LayersState {
    pub ui_layer_is_hot: bool,
}

impl LayersState {
    pub fn reset(&mut self) {
        self.ui_layer_is_hot = false;
    }
}

impl Default for TouchState {
    fn default() -> Self {
        TouchState {
            last_touch: Touch::None,
            touch: Touch::None,
            pos: Vec2f::ZERO,
            touch_start: Vec2f::ZERO,
            touch_current: Vec2f::ZERO,
        }
    }
}
