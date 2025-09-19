use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub side: Side,
    pub score: u32,
}

#[derive(Component, Debug)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub direction: Vec2,
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Component)]
pub struct Wall {
    pub side: WallSide,
}

#[derive(Debug)]
pub enum WallSide {
    Top,
    Bottom,
    Left,
    Right,
}
