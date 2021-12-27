use glam::Vec2;

// Traits
mod moves;

// Objects
mod city;

mod world_state;
#[cfg(test)]
mod world_state_tests;

use city::{Behavior, City};
use world_state::WorldState;

fn main() {
    println!("Hello, world!");

    let mut world_state = WorldState {
        cities: vec![City {
            position: Vec2::new(0.0, 0.0),
            direction: Vec2::new(1.0, 0.0),
            name: "FirstCity".to_string(),
            strength: 10f32,
            behavior: Behavior::AGGRESSIVE,
        }],
    };

    world_state = world_state.get_next_state();

    print!("{}", world_state);
}
