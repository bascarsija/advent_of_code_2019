pub mod panel {
    pub mod input {
        use advent_of_code_2019::input;
        use crate::panel::{PathVector, Panel};

        pub fn path_from_string(path_str: &str) -> Vec<PathVector> {
            path_str.split(",").map(|vec_str| {
                match PathVector::from_string(vec_str) {
                    Some(vec) => vec,
                    None => panic!("unable to parse path vector: {}", vec_str)
                }
            }).collect()
        }

        fn input_paths() -> Vec<Vec<PathVector>> {
            input::lines_from_arg_file().map(|line| {
                path_from_string(&line.unwrap())
            }).collect()
        }

        pub fn panel_with_input_paths() -> Panel {
            let mut panel = Panel::new();
            let mut wire_id = 0;
            for path in input_paths() {
                panel.add_wire_path(path.as_slice(), wire_id);

                wire_id += 1;
            }

            panel
        }
    }

    use std::collections::HashMap;
    use std::collections::hash_map::Entry;

    #[derive(Debug)]
    pub enum Direction {
        UP, DOWN, LEFT, RIGHT
    }

    impl Direction {
        fn from_string(str: &str) -> Option<Direction> {
            match str {
                "U" => Some(Direction::UP),
                "D" => Some(Direction::DOWN),
                "L" => Some(Direction::LEFT),
                "R" => Some(Direction::RIGHT),
                _ => None
            }
        }

        fn increment_point(&self, point: (i16,i16)) -> (i16,i16) {
            let (point_x, point_y) = point;

            match self {
                Direction::UP => (point_x, point_y + 1),
                Direction::DOWN => (point_x, point_y - 1),
                Direction::LEFT => (point_x - 1, point_y),
                Direction::RIGHT => (point_x + 1, point_y),
            }
        }
    }

    #[derive(Debug)]
    pub struct PathVector {
        direction: Direction,
        distance: u16
    }

    impl PathVector {
        pub fn from_string(str: &str) -> Option<PathVector> {
            if str.len() < 2 {
                None
            }
            else {
                Direction::from_string(&str[0..1]).and_then(|dir| {
                    match str[1..].parse::<u16>() {
                        Ok(dist) => Some(PathVector { direction: dir, distance: dist }),
                        Err(_) => None
                    }
                })
            }
        }
    }

    pub struct Panel {
        coords: HashMap<(i16,i16), HashMap<u8,u32>>,
        min_distance: Option<u16>,
        min_combined_path_len: Option<u32>
    }

    impl Panel {
        const NUM_WIRES: usize = 2;

        pub fn new() -> Panel {
            Panel {
                coords: HashMap::new(),
                min_distance: None,
                min_combined_path_len: None
            }
        }

        pub fn min_distance(&self) -> Option<u16> { self.min_distance }
        pub fn min_combined_path_length(&self) -> Option<u32> { self.min_combined_path_len }

        fn distance((point_x, point_y): (i16,i16)) -> u16 {
            (point_x.abs() + point_y.abs()) as u16
        }

        fn set_wire_path_point(&mut self, point: (i16,i16), wire_id: u8, path_len: u32) {
            //println!("setting point for wire id = {}: {:?}", wire_id, point);

            let wires = match self.coords.entry(point) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(HashMap::new())
            };

            if let Entry::Vacant(entry) = wires.entry(wire_id) {
                entry.insert(path_len);
            }

            if wires.len() == Panel::NUM_WIRES {
                let point_dist = Panel::distance(point);
                match self.min_distance {
                    Some(min_dist) => {
                        if point_dist < min_dist {
                            self.min_distance = Some(point_dist);
                        }
                    },
                    None => self.min_distance = Some(point_dist)
                }

                let point_combined_len = wires.values().fold(0, |tot, curr| tot + *curr);
                match self.min_combined_path_len {
                    Some(min_combined_len) => {
                        if point_combined_len < min_combined_len {
                            self.min_combined_path_len = Some(point_combined_len);
                        }
                    },
                    None => self.min_combined_path_len = Some(point_combined_len)
                }
            }
        }

        pub fn add_wire_path(&mut self, path: &[PathVector], wire_id: u8) {
            let mut point = (0,0);
            let mut path_len = 0;
            for vec in path {
                //println!("vector for wire id = {}: {:?}", wire_id, vec);
                for _ in 0 .. vec.distance {
                    //print!("cursor: {:?} -> ", point);

                    point = vec.direction.increment_point(point);
                    path_len += 1;

                    //println!("{:?}", point);

                    self.set_wire_path_point(point, wire_id, path_len);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::panel::{Panel, input};

        fn add_wire_paths(path_0: &str, path_1: &str) -> Panel {
            let mut panel = Panel::new();

            panel.add_wire_path(input::path_from_string(path_0).as_slice(), 0);
            panel.add_wire_path(input::path_from_string(path_1).as_slice(), 1);

            panel
        }

        fn test_min_distance(path_0: &str, path_1: &str, expected_dist: u16) {
            let panel = add_wire_paths(path_0, path_1);

            assert_eq!(panel.min_distance(), Some(expected_dist));
        }

        #[test]
        fn supplied_min_distance_test_case_1() {
            test_min_distance("R8,U5,L5,D3", "U7,R6,D4,L4", 6);
        }

        #[test]
        fn supplied_min_distance_test_case_2() {
            test_min_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
                159
            );
        }

        #[test]
        fn supplied_min_distance_test_case_3() {
            test_min_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135
            );
        }

        fn test_combined_length(path_0: &str, path_1: &str, expected_len: u32) {
            let panel = add_wire_paths(path_0, path_1);

            assert_eq!(panel.min_combined_path_length(), Some(expected_len));
        }

        #[test]
        fn supplied_combined_length_test_case_1() {
            test_combined_length(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
                610
            );
        }

        #[test]
        fn supplied_combined_length_test_case_2() {
            test_combined_length(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410
            );
        }
    }
}
