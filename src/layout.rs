use crate::commands::Vec2f;

pub struct StackLayout {
    x: f32,
    y: f32,
    initial_x: f32,
    initial_y: f32,
}

impl StackLayout {
    pub fn new(x: f32, y: f32) -> Self {
        StackLayout { x, y, initial_x: x, initial_y: y }
    }

    pub fn push_vertical(&mut self, height: f32) {
        self.y += height;
    }

    pub fn reset_vertical(&mut self) {
        self.y = self.initial_y;
    }

    pub fn push_horizontal(&mut self, width: f32) {
        self.x += width;
    }

    pub fn reset_horizontal(&mut self) {
        self.x = self.initial_x;
    }

    pub fn pos(&self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }
}
