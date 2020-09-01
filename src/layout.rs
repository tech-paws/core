use crate::commands::Vec2f;

pub struct StackLayout {
    x: f32,
    y: f32,
}

impl StackLayout {
    pub fn new(x: f32, y: f32) -> Self {
        StackLayout { x, y }
    }

    pub fn push_vertical(&mut self, height: f32) {
        self.x = 0.;
        self.y += height;
    }

    pub fn push_horizontal(&mut self, width: f32) {
        self.x += width;
        self.y = 0.;
    }

    pub fn pos(&self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }
}
