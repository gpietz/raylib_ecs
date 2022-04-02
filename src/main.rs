extern crate bevy_ecs;
extern crate raylib;

use bevy_ecs::prelude::{Schedule, Stage, SystemStage, World};
use bevy_ecs::system::{NonSend, ResMut};
use raylib::{RaylibHandle, RaylibThread};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let (raylib_handle, raylib_thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Raylib Bevy ECS")
        .build();

    let mut world = World::new();
    world.insert_resource(raylib_handle);
    world.insert_non_send(raylib_thread);

    let mut schedule = Schedule::default();
    schedule.add_stage("render", SystemStage::single_threaded().with_system(render));

    loop {
        if should_close(&mut world) {
            break;
        }
        schedule.run(&mut world);
    }
}

fn should_close(world: &mut World) -> bool {
    world
        .get_resource::<RaylibHandle>()
        .unwrap()
        .window_should_close()
}

fn render(mut rl_handle: ResMut<RaylibHandle>, rl_thread: NonSend<RaylibThread>) {
    let mut draw = rl_handle.begin_drawing(&rl_thread);
    draw.clear_background(Color::BLACK);
    draw.draw_fps(10, 10);
}
