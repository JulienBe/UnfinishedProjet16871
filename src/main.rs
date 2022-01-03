mod geometry;

use bevy::prelude::*;
use geo::{LineString, Point, Polygon};
use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;

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

struct Tile {
    color: Color,
    w: f32,
    h: f32,
    x: f32,
    y: f32,
}

struct Room {
    section: Polygon<f32>,
    tiles: Vec<Tile>,
}

impl Room {
    pub fn new(bottom_left: Point<f32>, top_right: Point<f32>) -> Self {
        let tile_width: f32 = 8.0;
        let tile_height: f32 = 4.0;
        let room_boundaries = Polygon::new(
            LineString::from(vec![(bottom_left.x(), bottom_left.y()), (bottom_left.x(), top_right.y()), (top_right.x(), top_right.y()), (bottom_left.x(), top_right.y())]),
            vec![],
        );
        let bound_rect = room_boundaries.bounding_rect().unwrap();
        let mut room_tiles: Vec<Tile> = Vec::new();

        for x in 0..bound_rect.width() as i32 {
            for y in 0..bound_rect.height() as i32 {
                let x1: f32 = (x as f32 * tile_width) + bound_rect.min().x;
                let x2: f32 = x1 + tile_width;
                let y1: f32 = (y as f32 * tile_height) + bound_rect.min().y;
                let y2: f32 = y1 + tile_height;
                let tile_poly = Polygon::new(LineString::from(vec![(x1, y1), (x2, y1), (x2, y2), (x1, y2)]), vec![]);
                if room_boundaries.contains(&tile_poly) {
                    let tile = Tile {
                        color: Color::rgb(0.5, 0.5, 0.5),
                        w: tile_width,
                        h: tile_height,
                        x: x1,
                        y: y1,
                    };
                    room_tiles.push(tile);
                }
            }
        }
        Room {
            section: room_boundaries,
            tiles: room_tiles,
        }
    }
}

fn setup(mut cmds: Commands, mut mats: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut window = windows.get_primary_mut().unwrap();
    //window.set_position(IVec2::new(0, 0));

    let room = Room::new(Point::new(0.0, 0.0), Point::new(200.0, 200.0));

    let bottom = -window.height() / 2.0;

    // The big pink thing
    cmds.spawn_bundle(SpriteBundle {
        material: mats.add(Color::rgb(1.0, 0.7, 0.7).into()),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + 50.0, 10.0),
            ..Default::default()
        },
        sprite: Sprite::new(Vec2::new(200.0, 100.0)),
        ..Default::default()
    });
    // the tiles
    for tile in room.tiles {
        println!("x: {}, y: {}, w: {}, h: {}", tile.x, tile.y, tile.w, tile.h);
        cmds.spawn_bundle(SpriteBundle{
            material: mats.add(tile.color.into()),
            transform: Transform {
                translation: Vec3::new(tile.x, tile.y, 1.0),
                ..Default::default()
            },
            sprite: Sprite::new(Vec2::new(tile.w, tile.h)),
            ..Default::default()
        });
    }
}