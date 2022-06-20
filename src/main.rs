#![allow(non_snake_case)]

use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

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
    .add_plugins(DefaultPlugins)
    .run();
}