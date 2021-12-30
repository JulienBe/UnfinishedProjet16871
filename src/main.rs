mod geometry;

use bevy::prelude::*;
use crate::geometry::geometry::{create_section, Point};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "DD".to_string(),
            width: 240.0 * 3.0,
            height: 160.0 * 3.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut cmds: Commands, mut mats: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut window = windows.get_primary_mut().unwrap();
    //window.set_position(IVec2::new(0, 0));

    let room = create_section(Point::new(10, 10), 20, 20);

    let bottom = -window.height() / 2.0;
    cmds.spawn_bundle(SpriteBundle {
        material: mats.add(Color::rgb(1.0, 0.7, 0.7).into()),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + 50.0, 10.0),
            ..Default::default()
        },
        sprite: Sprite::new(Vec2::new(200.0, 100.0)),
        ..Default::default()
    });
}