#![allow(non_snake_case)]
#![allow(clippy::redundant_field_names)]

use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.2, 0.3);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod player;
use player::PlayerPlugin;

fn main() {
    println!("Hello, world!!");
    App::new()
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
        width: 1600.0,
        height: 900.0,
        title: "Cool".to_string(),
        resizable: true,
        ..Default::default()
    })
    .add_startup_system(spawn_camera)
    .add_plugin(PlayerPlugin)
    .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
    .add_plugins(DefaultPlugins)
    .run();
}



fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(9.0), 16, 16, Vec2::splat(2.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}