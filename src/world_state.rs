


    use glam::{Vec2};

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct City {
        pub position: Vec2,
        pub direction: Vec2,
    }

    trait Moves {
        fn move_me(&self) -> Vec2;
    }

    impl Moves for City {
        fn move_me(&self) -> Vec2 {
            let next_position = self.position + self.direction;

            return next_position;
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct WorldState {
        pub cities: Vec<City>
    }

    impl WorldState {
        pub fn get_next_state(&self) -> WorldState {
            let collision_unchecked_cities: Vec<City> = self.cities.iter().map(|city| {
                let next_position = city.move_me();

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
