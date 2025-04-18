use bevy::prelude::*;

use crate::model::GameState;

use super::systems::player_input_system;

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        // Register systems
        // app.add_systems(Update, keyboard_input.in_set(AppSet::RecordInput))
        //     .add_observer(handle_player_actions);

        app.add_systems(Update, player_input_system.run_if(in_state(GameState::PlayerTurn)));
    }
}
