use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::{Vec2, Vec3},
    render::mesh::Mesh,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BRICK_SIZE: Vec2 = Vec2 { x: 100.0, y: 30.0 };

#[derive(Component)]
struct Brick;

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Sprite {
            color: BRICK_COLOR,
            ..default()
        },
        Transform {
            translation: Vec2::new(0.0, 0.0).extend(0.0),
            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
            ..default()
        },
        Brick,
    ));
}
