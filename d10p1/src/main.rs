use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;
use std::path::Path;

/// Steps
/// 1. get map of heights in grid
/// 2. find each trailhead
/// 3. 'walk' each trailhead to the top
///     - for each up/down/left/right find valid paths
///

fn main() {
    println!("Hello, world!");

    let topo_map = get_topographical_map_from_file(Path::new("input"));

    let mut total_scores: i32 = 0;
    for (y, row) in topo_map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                total_scores += walk((y as i32, x as i32), -1, &topo_map, HashSet::new()).len() as i32;
            }
        };
    };

    dbg!(total_scores);
}

enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match self {
            Self::U => (-1, 0),
            Self::D => (1, 0),
            Self::L => (0, -1),
            Self::R => (0, 1),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::U, Self::D, Self::L, Self::R]
    }
}

fn get_topographical_map_from_file(p: &Path) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();

    for d in read(p).unwrap().lines() {
        map.push(d.unwrap().chars().map(|n| n.to_digit(10).unwrap() as i32).collect());
    }

    map
}

fn walk(location: (i32, i32), prev_height: i32, map: &Vec<Vec<i32>>, count: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
   // off map
    if location.0 >= map.len() as i32 || location.0 < 0 || location.1 >= map[0].len() as i32 || location.1 < 0 {
        return count
    } 

    let current_height = map[location.0 as usize][location.1 as usize];

    // too steep
    if current_height != prev_height + 1 {
        return count
    }

    // at peak
    if current_height == 9 {
        let mut new_count = count.clone();
        new_count.insert(location);
        return new_count
    }
    

    let mut new_count = count;
    for direction in Direction::all() {
        new_count = walk(
            (location.0 + direction.value().0, location.1 + direction.value().1),
            current_height,
            map,
            new_count,
        );
    };

    new_count
}

