use bevy::prelude::*;
use rand::Rng;

use crate::model::{
    components::{Actor, Player, Position, Renderable, TerrainType, TurnActor},
    resources::{CurrentMap, TurnSystem},
    utils::spawn_ascii_entity,
    ModelConstants,
};

pub fn spawn_player(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    asset_server: Res<AssetServer>,
    mut turn_system: ResMut<TurnSystem>,
    terrain_query: Query<&TerrainType>,
) {
    // Find a valid floor tile for the player
    let mut valid_positions = Vec::new();
    for y in 1..ModelConstants::MAP_HEIGHT - 1 {
        for x in 1..ModelConstants::MAP_WIDTH - 1 {
            if let Some(terrain_entity) = current_map.get_terrain(Position::new(x as i32, y as i32))
            {
                if let Ok(terrain_type) = terrain_query.get(terrain_entity) {
                    if *terrain_type == TerrainType::Floor {
                        valid_positions.push((x as i32, y as i32));
                    }
                }
            }
        }
    }

    // Choose a random position
    let mut rng = rand::rng();
    let (x, y) = valid_positions[rng.random_range(0..valid_positions.len())];

    let player_position = Position::new(x, y);
    let player_id = spawn_ascii_entity(
        &mut commands,
        &asset_server,
        Some(Position::new(x, y)),
        Renderable {
            glyph: '@',
            color: Color::srgb(1.0, 1.0, 0.0), // Yellow
        },
        1.0,
    );

    commands.entity(player_id).insert((
        Player,
        TurnActor {
            speed: 100,
            next_turn_time: 0, // Player goes first
        },
    ));

    // Spawn an enemy
    let (x, y) = valid_positions[rng.random_range(0..valid_positions.len())];
    let actor_position = Position::new(x, y);
    let actor_id = spawn_ascii_entity(
        &mut commands,
        &asset_server,
        Some(actor_position),
        Renderable {
            glyph: 'E',
            color: Color::srgb(1.0, 0.0, 0.0), // Red
        },
        1.0,
    );

    commands.entity(actor_id).insert((
        Actor,
        TurnActor {
            speed: 120, // Enemy is slower
            next_turn_time: 0,
        },
    ));

    current_map.set_actor(player_position, Some(player_id));
    current_map.set_actor(actor_position, Some(actor_id));

    let current_time = turn_system.current_time();
    turn_system.schedule_turn(player_id, current_time);
    turn_system.schedule_turn(actor_id, current_time);
}
