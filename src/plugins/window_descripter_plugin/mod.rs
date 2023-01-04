// Contains all of the settings for the window.
use bevy::{prelude::*, window::PresentMode, app::PluginGroupBuilder};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
let mut WINDOW_DESCRIPTOR: WindowDescriptor{} = WindowDescriptor::default();

pub struct WindowDescripterPlugin;

impl Plugin for WindowDescripterPlugin {

    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(
            Color::hex("FF00FF00").unwrap(),
        ));
    }

}

