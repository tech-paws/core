use specs::System;

pub struct DemoRenderSystem;

impl<'a> System<'a> for DemoRenderSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        println!("Hello World!");
    }
}
