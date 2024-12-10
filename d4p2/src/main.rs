use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::cmp::min;

fn main() {
    let input_path = Path::new("input");
    let map = get_input_as_char_map(input_path);

    let found_count = find(map);

    dbg!(found_count);
}

fn get_input_as_char_map(path: &Path) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for d in fs::read(path).unwrap().lines() {
        map.push(d.unwrap().chars().collect());
    }

    map
}

#[derive(Debug)]
enum Direction {
    U,
    RU,
    R,
    RD,
    D,
    LD,
    L,
    LU,
}

impl Direction {
    pub fn value(&self) -> [i32; 2] {
        match *self {
            Direction::U => [-1, 0],
            Direction::RU => [-1, 1],
            Direction::R => [0, 1],
            Direction::RD => [1, 1],
            Direction::D => [1, 0],
            Direction::LD => [1, -1],
            Direction::L => [0, -1],
            Direction::LU => [-1, -1],
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Direction::U,
            Direction::RU,
            Direction::R,
            Direction::RD,
            Direction::D,
            Direction::LD,
            Direction::L,
            Direction::LU,
        ]
    }
}

fn find(map: Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    let start: char = 'A';

    for (y, row) in map.iter().enumerate() {
        let mut start_pos: usize = 0;
        while let Some(next_x) = position_from(start_pos, row, &start) {
            let leg1 = 
                search_direction(&"MAS".to_string(), [y as i32 - 1, next_x as i32 - 1], &Direction::RD, &map) || 
                search_direction(&"MAS".to_string(), [y as i32 + 1, next_x as i32 + 1], &Direction::LU, &map);
            let leg2 = 
                search_direction(&"MAS".to_string(), [y as i32 - 1, next_x as i32 + 1], &Direction::LD, &map) || 
                search_direction(&"MAS".to_string(), [y as i32 + 1, next_x as i32 - 1], &Direction::RU, &map);

            match leg1 && leg2 {
                true => count += 1,
                false => ()
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
