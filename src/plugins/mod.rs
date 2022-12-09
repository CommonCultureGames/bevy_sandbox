mod splash_screen_plugin;
use bevy::{prelude::*, app::PluginGroupBuilder};



pub struct BevyTopDownRPGCorePlugins;

impl PluginGroup for BevyTopDownRPGCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        // Create the Plugin Group Builder
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
        // Add the basic plugins    
        .add(bevy::log::LogPlugin::default())
        .add(bevy::core::CorePlugin::default())
        .add(bevy::time::TimePlugin::default())
        .add(bevy::transform::TransformPlugin::default())
        .add(bevy::hierarchy::HierarchyPlugin::default())
        .add(bevy::diagnostic::DiagnosticsPlugin::default())
        .add(bevy::input::InputPlugin::default())
        .add(bevy::window::WindowPlugin::default())
        // add the Custom Plugins
        .add(splash_screen_plugin::SplashScreenPlugin);
        #[cfg(feature = "bevy_asset")]
        {
            group = group.add(bevy::asset::AssetPlugin::default());
        }

        #[cfg(feature = "debug_asset_server")]
        {
            group = group.add(bevy::asset::debug_asset_server::DebugAssetServerPlugin::default());
        }

        #[cfg(feature = "bevy_scene")]
        {
            group = group.add(bevy::scene::ScenePlugin::default());
        }

        #[cfg(feature = "bevy_winit")]
        {
            group = group.add(bevy::winit::WinitPlugin::default());
        }

        #[cfg(feature = "bevy_render")]
        {
            group = group
                .add(bevy::render::RenderPlugin::default())
                // NOTE: Load this after renderer initialization so that it knows about the supported
                // compressed texture formats
                .add(bevy::render::texture::ImagePlugin::default());
        }

        #[cfg(feature = "bevy_core_pipeline")]
        {
            group = group.add(bevy::core::pipeline::CorePipelinePlugin::default());
        }

        #[cfg(feature = "bevy_sprite")]
        {
            group = group.add(bevy::sprite::SpritePlugin::default());
        }

        #[cfg(feature = "bevy_text")]
        {
            group = group.add(bevy::text::TextPlugin::default());
        }

        #[cfg(feature = "bevy_ui")]
        {
            group = group.add(bevy::ui::UiPlugin::default());
        }

        #[cfg(feature = "bevy_pbr")]
        {
            group = group.add(bevy::pbr::PbrPlugin::default());
        }

        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
        #[cfg(feature = "bevy_gltf")]
        {
            group = group.add(bevy::gltf::GltfPlugin::default());
        }

        #[cfg(feature = "bevy_audio")]
        {
            group = group.add(bevy::audio::AudioPlugin::default());
        }

        #[cfg(feature = "bevy_gilrs")]
        {
            group = group.add(bevy::gilrs::GilrsPlugin::default());
        }

        #[cfg(feature = "bevy_animation")]
        {
            group = group.add(bevy::animation::AnimationPlugin::default());
        }

        group
    }
}