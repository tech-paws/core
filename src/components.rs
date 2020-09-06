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
            pos: Vec2f::zero(),
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
    pub touch: Touch,
    pub touch_start: Vec2f,
    pub touch_current: Vec2f,
}

impl Default for TouchState {
    fn default() -> Self {
        TouchState {
            touch: Touch::None,
            touch_start: Vec2f::zero(),
            touch_current: Vec2f::zero(),
        }
    }
}
