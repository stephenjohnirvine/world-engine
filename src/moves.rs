use crate::world_state::WorldState;
use glam::Vec2;

pub trait Moves {
    fn move_me(&self) -> Vec2;
    fn direct_me(&self, state: &WorldState) -> Vec2;
}
