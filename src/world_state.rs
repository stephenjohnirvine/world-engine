use std::fmt;

use crate::city::City;
use crate::moves::Moves;

#[derive(Debug, PartialEq)]
pub struct WorldState {
    pub cities: Vec<City>,
}

impl fmt::Display for WorldState {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cities ({}):", self.cities.len());
        for c in self.cities.iter() {
            writeln!(f, " - {}", c);
        }
        writeln!(f);

        Ok(())
    }
}

impl WorldState {
    pub fn get_next_state(&self) -> WorldState {
        let moved_cities: Vec<City> = self
            .cities
            .iter()
            .map(|city| {
                let next_position = city.move_me();

                City {
                    position: next_position,
                    direction: city.direction,
                    behavior: city.behavior.clone(),
                    strength: city.strength,
                    name: city.name.clone(),
                }
            })
            .collect();

        let world = WorldState {
            cities: moved_cities,
        };

        let directed_cities: Vec<City> = world
            .cities
            .iter()
            .map(|city| {
                let next_direction = city.direct_me(&world);

                City {
                    position: city.position,
                    direction: next_direction,
                    behavior: city.behavior.clone(),
                    strength: city.strength,
                    name: city.name.clone(),
                }
            })
            .collect();

        WorldState {
            cities: directed_cities,
        }
    }
}
