use std::borrow::Cow;

use azalea_block::BlockState;
use azalea_core::{direction::Direction, position::BlockPos};

use crate::collision::{BlockWithShape, VoxelShape};



pub enum SupportType {
    Full,
    Center,
    Rigid
}

impl SupportType {
    pub fn is_supporting(&self, state: BlockState, pos: BlockPos, direction: Direction) -> bool {
        match self {
            SupportType::Full => {
                let shape = get_block_support_shape(state, pos);
                shape.is_surface_full(direction)
            },
            SupportType::Center => todo!(),
            SupportType::Rigid => todo!(),
        }
    }
}

fn get_block_support_shape(state: BlockState, pos: BlockPos) -> Cow<'static, VoxelShape> {
    match state {
        
        _ => {
            state.collision_shape(pos)
        }
    }
}