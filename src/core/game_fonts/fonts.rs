use bevy::prelude::*;

#[derive(Resource)]
pub struct GameFonts {
    pub kaisei_decol_bold: Handle<Font>,
    pub kaisei_decol_medium: Handle<Font>,
    pub kaisei_decol_regular: Handle<Font>,

    pub lora_font_bold: Handle<Font>,

    pub inter_bold: Handle<Font>,
}

impl FromWorld for GameFonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            kaisei_decol_bold: asset_server.load("fonts/kaisei_decol/KaiseiDecol-Bold.ttf"),
            kaisei_decol_medium: asset_server.load("fonts/kaisei_decol/KaiseiDecol-Medium.ttf"),
            kaisei_decol_regular: asset_server.load("fonts/kaisei_decol/KaiseiDecol-Regular.ttf"),
            inter_bold: asset_server.load("fonts/inter/static/Inter_28pt-Bold.ttf"),
            lora_font_bold: asset_server.load("fonts/lora_font/static/Lora-Medium.ttf"),
        }
    }
}
