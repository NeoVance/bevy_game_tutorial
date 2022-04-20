use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_asset_loader::{AssetLoaderPlugin, AssetCollection};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AssetLoaderPlugin::<GameAssets, _>::new(
            GameState::Loading,
            GameState::Menu,
        ));
    }
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Texture>,
}
