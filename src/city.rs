use glam::Vec2;

use crate::moves::Moves;
use crate::world_state::WorldState;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Behavior {
    AGGRESSIVE,
    DEFENSIVE,
}

#[derive(PartialEq, Debug)]
pub struct City {
    pub name: String,
    pub position: Vec2,
    pub direction: Vec2,
    pub strength: f32,
    pub behavior: Behavior,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.position.x, self.position.y)
    }
}

impl Moves for City {
    fn move_me(&self) -> Vec2 {
        self.position + self.direction
    }

    fn direct_me(&self, state: &WorldState) -> Vec2 {
        let visible: Vec<&City> = state
            .cities
            .iter()
            .filter(|&city| self.name != city.name)
            .filter(|&city| self.position.distance(city.position) < 10f32)
            .collect();

        let (smaller, bigger): (Vec<&City>, Vec<&City>) = visible
            .iter()
            .partition(|&city| self.strength < city.strength);

        match self.behavior {
            Behavior::AGGRESSIVE => match smaller.first() {
                None => self.direction,
                Some(city) => {
                    let dir = city.position - self.position;

                    dir.normalize()
                }
            },
            Behavior::DEFENSIVE => match bigger.first() {
                None => self.direction,
                Some(city) => {
                    let dir = self.position - city.position;

                    dir.normalize()
                }
            },
        }
    }
}
