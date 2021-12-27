#[cfg(test)]
mod world_state_tests {
    use super::super::world_state::*;

    use crate::city::{Behavior, City};
    use glam::Vec2;

    #[test]
    fn movement_test() {
        let sut = WorldState {
            cities: vec![City {
                position: Vec2::new(0.0, 0.0),
                direction: Vec2::new(1.0, 0.0),
                behavior: Behavior::AGGRESSIVE,
                strength: 10f32,
                name: "Bob".to_string(),
            }],
        };

        let output = sut.get_next_state();

        let expected = WorldState {
            cities: vec![City {
                position: Vec2::new(1.0, 0.0),
                direction: Vec2::new(1.0, 0.0),
                behavior: Behavior::AGGRESSIVE,
                strength: 10f32,
                name: "Bob".to_string(),
            }],
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn behavior_test() {
        let sut = WorldState {
            cities: vec![
                City {
                    position: Vec2::new(0.0, 0.0),
                    direction: Vec2::new(1.0, 0.0),
                    behavior: Behavior::AGGRESSIVE,
                    strength: 10f32,
                    name: "Strong".to_string(),
                },
                City {
                    position: Vec2::new(3.0, 1.0),
                    direction: Vec2::new(-1.0, 0.0),
                    behavior: Behavior::DEFENSIVE,
                    strength: 1f32,
                    name: "Weak".to_string(),
                },
            ],
        };

        let output = sut.get_next_state();

        let expected = WorldState {
            cities: vec![
                City {
                    position: Vec2::new(1.0, 0.0),
                    direction: Vec2::new(1.0, 1.0).normalize(),
                    behavior: Behavior::AGGRESSIVE,
                    strength: 10f32,
                    name: "Strong".to_string(),
                },
                City {
                    position: Vec2::new(2.0, 1.0),
                    direction: Vec2::new(1.0, 1.0).normalize(),
                    behavior: Behavior::DEFENSIVE,
                    strength: 1f32,
                    name: "Weak".to_string(),
                },
            ],
        };
        assert_eq!(output, expected);
    }
}
