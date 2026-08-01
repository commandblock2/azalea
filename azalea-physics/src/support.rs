use bevy_ecs::prelude::*;
use azalea_entity::{HasClientLoaded, LocalEntity, Physics, Position};
use azalea_world::{WorldName, Worlds};

use crate::collision::{calculate_supporting_block_at_current_pos};


pub fn update_main_supporting_block_pos(
    mut query: Query<(Entity, &Position, &WorldName, &mut Physics), (With<LocalEntity>, With<HasClientLoaded>)>,
    worlds: Res<Worlds>,
) {
    for (enity, position, world_name, mut physics) in &mut query {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        physics.supporting_ctx = calculate_supporting_block_at_current_pos(physics.on_ground(), **position, &physics, &world, enity);
    }
}