use azalea_core::position::Vec3;
use azalea_entity::{GroundContact, HasClientLoaded, LocalEntity, MovementResult, Physics, Position};
use azalea_world::{WorldName, Worlds};
use bevy_ecs::prelude::*;

use crate::collision::{world_collisions::{EntityCollisionContext, find_supporting_block}};

#[derive(PartialEq)]
pub enum SupportingBlockUpdate {
    LocalMovement,
    PositionUpdateFromServer,
}

pub fn update_main_supporting_block_pos(
    InMut(update_context): InMut<SupportingBlockUpdate>,
    mut query: Query<
        (Entity, &Position, &WorldName, &MovementResult, &mut Physics, &mut GroundContact),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (entity, position, world_name, movement_result, physics, mut ground_context) in &mut query {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        if !ground_context.on_ground() {
            ground_context.on_ground_no_support = false;
            continue;
        }

        let test_box = {
            let mut box_ = physics.bounding_box;
            box_.max.y = box_.min.y;
            box_.min.y = box_.min.y - 1e-6;
            box_
        }; // small volume under player foot

        let pos = find_supporting_block(
            &world,
            test_box,
            **position,
            EntityCollisionContext::of(Some(entity)),
        );

        let pos = if pos.is_some() || ground_context.on_ground_no_support {
            pos
        } else if *update_context == SupportingBlockUpdate::LocalMovement {
            let movement = movement_result.actual;
            let fallback_testbox = test_box.move_relative(Vec3 {
                x: -movement.x,
                y: 0.0,
                z: -movement.z,
            });
            find_supporting_block(
                &world,
                fallback_testbox,
                **position,
                EntityCollisionContext::of(Some(entity)),
            )
        } else {
            pos
        };

        
        ground_context.supporting_block = pos;
        ground_context.on_ground_no_support = pos.is_none();
    }
}
