use bevy::prelude::*;

use crate::model::components::{Renderable, TerrainType};

pub fn add_sprite_to_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_tiles: Query<(Entity, &TerrainType), Added<TerrainType>>,
) {
    for (entity, tile_type) in q_tiles.iter() {
        // commands.entity(entity).insert(Sprite {
        //     image: asset_server.load("terminal_32x32.png"),
        //     texture_atlas: Some(TextureAtlas {
        //         layout: asset_server.add(TextureAtlasLayout::from_grid(
        //             UVec2::splat(32),
        //             16,
        //             16,
        //             None,
        //             None,
        //         )),
        //         index: match tile_type {
        //             TerrainType::Floor => 250,
        //             TerrainType::Wall => 35,
        //         },
        //     }),
        //     custom_size: Some(Vec2::splat(ViewConstants::TILE_SIZE)),
        //     anchor: Anchor::BottomLeft,
        //     ..Default::default()
        // });

        let font = asset_server.load("fonts/FiraMono-Medium.ttf");

        let text_style = TextFont { font: font.clone(), font_size: 25.0, ..default() };

        let renderable = match tile_type {
            TerrainType::Floor => Renderable { glyph: '.', color: Color::srgb(0.5, 0.5, 0.5) }, /* #808080 */
            TerrainType::Wall => Renderable { glyph: '#', color: Color::srgb(0.7, 0.7, 0.7) },  /* #b3b3b3 */
            TerrainType::Door => Renderable { glyph: '+', color: Color::srgb(0.65, 0.4, 0.1) }, /* #a66719 */
            TerrainType::UpStairs => Renderable { glyph: '<', color: Color::srgb(1.0, 1.0, 0.0) }, /* #ffff00 */
            TerrainType::DownStairs => Renderable { glyph: '>', color: Color::srgb(1.0, 1.0, 0.0) }, /* #ffff00 */
        };

        commands.entity(entity).insert((
            text_style,
            TextColor(renderable.color),
            Text2d::new(renderable.glyph.to_string()),
            renderable,
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::default(),
        ));
    }
}
