use std::sync::Mutex;

use crate::commands::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RENDER_STATE: Mutex<RenderState> = Mutex::new(RenderState::default());
}

pub struct RenderState {
    offset: usize,
    text_sizes: Vec<Vec2f>,
}

impl RenderState {
    pub fn default() -> Self {
        RenderState {
            offset: 0,
            text_sizes: Vec::new(),
        }
    }

    pub fn next_text_size(&mut self) -> Vec2f {
        self.offset += 1;

        if self.offset - 1 >= self.text_sizes.len() {
            Vec2f::zero()
        }
        else {
            self.text_sizes[self.offset - 1]
        }
    }

    pub fn bump_cursor(&mut self) {
        self.offset = 0;
    }

    pub fn clear(&mut self) {
        self.text_sizes.clear();
        self.offset = 0;
    }

    pub fn push(&mut self, size: Vec2f) {
        self.text_sizes.push(size);
    }
}
