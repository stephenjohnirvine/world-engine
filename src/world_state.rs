use glam::Vec2;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct City {
    pub position: Vec2,
    pub direction: Vec2,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.position.x, self.position.y)
    }
}

trait Moves {
    fn move_me(&self) -> Vec2;
}

impl Moves for City {
    fn move_me(&self) -> Vec2 {
        self.position + self.direction
    }
}

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
        let collision_unchecked_cities: Vec<City> = self
            .cities
            .iter()
            .map(|city| {
                let next_position = city.move_me();

                City {
                    position: next_position,
                    direction: city.direction,
                }
            })
            .collect();

        WorldState {
            cities: collision_unchecked_cities,
        }
    }
}
