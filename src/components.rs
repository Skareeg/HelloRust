use specs::prelude::*;
use rltk::{RGB};

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
pub struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB
}

#[derive(Component)]
pub struct Player {}