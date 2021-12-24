use glam::{Vec2};

fn main() {
    println!("Hello, world!");

    let mut world_state = WorldState {
        cities: vec![
            City { 
               position: Vec2::new(0.0, 0.0),
               direction: Vec2::new(1.0, 0.0)
            }
        ]
    };

    world_state = world_state.getNextState();

}

struct City {
    position: Vec2,
    direction: Vec2,
}

trait Moves {
    fn moveMe(&self) -> Vec2;
}

impl Moves for City {
    fn moveMe(&self) -> Vec2 {
        let next_position = self.position + self.direction;

        return next_position;
    }
}

struct WorldState {
    cities: Vec<City>
}

impl WorldState {
    fn getNextState(&self) -> WorldState {
        let collision_unchecked_cities: Vec<City> = self.cities.iter().map(|city| {
            let next_position = city.moveMe();

            return City {
                position: next_position,
                direction: city.direction
            };
        }).collect();

        return WorldState {
            cities: collision_unchecked_cities
        }
    }
}
