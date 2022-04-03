extern crate bevy_ecs;
extern crate raylib;

use std::collections::hash_map::Keys;
use bevy_ecs::prelude::{Query, Schedule, Stage, SystemStage, World, Component};
use bevy_ecs::system::{NonSend, ResMut};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};
use raylib::consts::KeyboardKey::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

#[derive(Component)]
struct Position {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Raylib Bevy ECS")
        .build();
    raylib_handle.set_target_fps(60);

    let mut world = World::new();
    world.insert_resource(raylib_handle);
    world.insert_non_send(raylib_thread);
    world.spawn().insert(Position { x: 100, y: 100 });

    let mut schedule = Schedule::default();
    schedule.add_stage(
        "render",
        SystemStage::single_threaded()
            .with_system(input)
            .with_system(render),
    );

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

fn input(
    mut rl_handle: ResMut<RaylibHandle>,
    mut query: Query<&mut Position>
) {
    let mut position = query.single_mut();
    if rl_handle.is_key_down(KEY_LEFT ) && position.x > 0 {
        position.x -= 4;
    }
    if rl_handle.is_key_down(KEY_RIGHT ) && position.x + 40 < rl_handle.get_screen_width() {
        position.x += 4;
    }
    if rl_handle.is_key_down(KEY_UP) && position.y > 0 {
        position.y -= 4;
    }
    if rl_handle.is_key_down(KEY_DOWN) && position.y + 40 < rl_handle.get_screen_height() {
        position.y += 4;
    }
}

fn render(
    mut rl_handle: ResMut<RaylibHandle>,
    rl_thread: NonSend<RaylibThread>,
    query: Query<&Position>
) {
    let mut draw = rl_handle.begin_drawing(&rl_thread);
    draw.clear_background(Color::BLACK);
    draw.draw_fps(10, 10);

    let position = query.single();
    draw.draw_rectangle(position.x, position.y, 40, 40, Color::YELLOW);
}
