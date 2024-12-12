use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input_path = Path::new("input");
    let map = get_input_as_char_map(input_path);

    let mut guard = find_guard_location_and_initial_direction(&map);

    let steps = guard.walk(&map);

    println!("sup");
}

fn get_input_as_char_map(path: &Path) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for d in fs::read(path).unwrap().lines() {
        map.push(d.unwrap().chars().collect());
    }

    map
}

fn find_guard_location_and_initial_direction(map: &Vec<Vec<char>>) -> Guard {
    for (idx, l) in map.iter().enumerate() {
        if let Some(val) = l.iter().position(|v| *v != Location::Obstruction.value() && *v != Location::Navigable.value()) {
            let facing = match map[idx][val] {
                '^' => Direction::U,
                '>' => Direction::R,
                'v' => Direction::D,
                '<' => Direction::L,
                _ => panic!("what the hell is even that??")
            };
            return Guard {
                steps: HashSet::new(),
                facing,
                location: [idx as i32, val as i32],
            }
        }
    };
    panic!("No guard was found")
}

struct Guard {
    steps: HashSet<[i32;2]>,
    facing: Direction,
    location: [i32;2],  // [y, x]
}

impl Guard {
    pub fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::U => Direction::R,
            Direction::R => Direction::D,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
        };
        // TODO: Mabye walk forward?
    }

    pub fn walk_forward(&mut self, map: &Vec<Vec<char>>) -> Location {
        let test_location = [self.location[0] + self.facing.value()[0], self.location[1] + self.facing.value()[1]];

        // if off map
        if test_location[0] >= map.len() as i32 || test_location[1] >= map[0].len() as i32 {
            return Location::OffMap;
        };

        // if obstruction
        if map[test_location[0] as usize][test_location[1] as usize] == Location::Obstruction.value() {
            return Location::Obstruction;
        }

        // otherwise
        self.location = test_location;
        self.steps.insert(test_location);

        Location::Navigable
    }

    pub fn walk(&mut self, map: &Vec<Vec<char>>) -> HashSet<[i32;2]> {
        loop {
            match self.walk_forward(map) {
                Location::OffMap => break,
                Location::Obstruction => self.turn_right(),
                _ => ()
            }
        }

        self.steps.clone()
    }
}

#[derive(Debug)]
enum Location {
    OffMap,
    Obstruction,
    Navigable,
}

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    pub fn value(&self) -> [i32; 2] {
        match *self {
            Direction::U => [-1, 0],
            Direction::R => [0, 1],
            Direction::D => [1, 0],
            Direction::L => [0, -1],
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Direction::U,
            Direction::R,
            Direction::D,
            Direction::L,
        ]
    }
}

impl Location {
    pub fn value(&self) -> char {
        match *self {
            Location::Obstruction => '#',
            Location::OffMap => ' ',
            Location::Navigable => '.',
        }
    }
}

fn find(term: &String, map: Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    let start: char = term.chars().next().unwrap();

    for (y, row) in map.iter().enumerate() {
        let mut start_pos: usize = 0;
        while let Some(next_x) = position_from(start_pos, row, &start) {
            for dir in Direction::all().iter() {
                match search_direction(&term[0..].to_string(), [y as i32, next_x as i32], dir, &map) {
                    true => count += 1,
                    false => ()
                };
            }
            start_pos = next_x + 1;
            if start_pos >= row.len() {
                break;
            }
            //println!("y - {:?}  x - {:?}  start - {:?}", y, next_x, start_pos);
        }
    }
    count
}

fn position_from<T: std::cmp::PartialEq + std::fmt::Debug>(pos: usize, searchable: &Vec<T>, target: &T) -> Option<usize> {
    match searchable[pos..].iter().position(|x| x == target) {
        Some(n) => Some(pos + n),
        _ => None
    }
}

fn search_direction(
    term: &String, 
    coords: [i32;2], 
    direction: &Direction, 
    map: &Vec<Vec<char>>
) -> bool {

    if term.is_empty() {
        return true
    }

    if coords[0] < 0 || coords[1] < 0 {
        return false
    }

    if coords[0] >= map[0].len() as i32 || coords[1] >= map.len() as i32 {
        return false
    }

    //println!("dir - {:?}  term - {:?}  stuff - {:?}", direction, term, map[coords[0] as usize][coords[1] as usize]);
    match term.chars().next() {
        Some(n) if n == map[coords[0] as usize][coords[1] as usize] => {
            search_direction(
                &term[1..].to_string(), 
                [coords[0] + direction.value()[0], coords[1] + direction.value()[1]], 
                direction, 
                map,
            )
        },
        _ => false,
    }
}
