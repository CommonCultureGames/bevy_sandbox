mod splash_screen_plugin;
mod window_descriptor_plugin;
use bevy::{prelude::*, app::PluginGroupBuilder};

use window_descriptor_plugin::BevyTopDownRPGCoreWindowPlugin;

pub struct BevyTopDownRPGCorePlugins;

impl PluginGroup for BevyTopDownRPGCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        // Create the Plugin Group Builder
        println!("Plugin Group Started");
        let group = PluginGroupBuilder::start::<Self>();
        group.add(BevyTopDownRPGCoreWindowPlugin::default())
    }
}