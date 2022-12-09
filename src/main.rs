mod testcode;
mod plugins;
use bevy::prelude::*;
use plugins::BevyTopDownRPGCorePlugins;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    One,
}

fn main() {
//    testcode::load_font::main();
    App::new()
        .insert_resource(ClearColor(
            Color::hex("FF00FF00").unwrap(),
        ))
        .add_plugins(BevyTopDownRPGCorePlugins);

}