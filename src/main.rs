use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
#[macro_use]
extern crate specs_derive;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB
}

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World!");
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();
    let mut gs = State {
        ecs: World::new()
    };
    rltk::main_loop(context, gs);
}
