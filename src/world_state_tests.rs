#[cfg(test)]
mod world_state_tests {
    use super::super::world_state::{*};

    use glam::{Vec2};

    #[test]
    fn get_next_state_test() {
        let sut = WorldState {
            cities: vec![
                City { 
                   position: Vec2::new(0.0, 0.0),
                   direction: Vec2::new(1.0, 0.0)
                }
            ]
        };

        let output = sut.get_next_state();

        let expected = WorldState {
            cities: vec![
                City { 
                   position: Vec2::new(1.0, 0.0),
                   direction: Vec2::new(1.0, 0.0)
                }
            ]
        };
        
        assert_eq!(output, expected);
    }
}