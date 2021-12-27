use glam::Vec2;

#[cfg(test)]
mod world_state_tests;

mod world_state;

use world_state::{City, WorldState};

fn main() {
    println!("Hello, world!");

    let mut world_state = WorldState {
        cities: vec![City {
            position: Vec2::new(0.0, 0.0),
            direction: Vec2::new(1.0, 0.0),
        }],
    };

    world_state = world_state.get_next_state();
}
