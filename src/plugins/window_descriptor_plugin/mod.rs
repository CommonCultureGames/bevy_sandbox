

// Contains all of the settings for the window.
use bevy::{prelude::*, window::{PresentMode}};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
#[derive(Debug)]
pub struct BevyTopDownRPGCoreWindowPlugin{
    window: WindowDescriptor
}

impl Plugin for BevyTopDownRPGCoreWindowPlugin {
    fn build(&self, app: &mut App) {
        println!("Window Plugin Started");
        println!("{:?}",self);
        app.add_plugin(WindowPlugin {
            window: self.window.clone(),
            ..Default::default()
        });
    }
}

impl Default for BevyTopDownRPGCoreWindowPlugin {
    fn default() -> Self {
        BevyTopDownRPGCoreWindowPlugin {
            window: WindowDescriptor {
                title: "BevyTopDownRPGCORE".to_string(),
                width: WIDTH,
                height: HEIGHT,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
        }
    }
}