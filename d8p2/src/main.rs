use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufRead;
use std::path::Path;

use num::integer::gcd;

fn main() {
    println!("Hello, world!");
    let map = get_input_as_char_map(Path::new("input"));
    let ms = (map.len() as i32, map[0].len() as i32);

    let ants = find_antannaes(&map);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_freq, nodes) in ants.iter() {
        for n in nodes.iter() {
            for o in nodes.iter() {
                antinodes.extend(n.find_antinodes(o, ms));
            }
        }
    }

    dbg!(antinodes.len());
}

fn get_input_as_char_map(path: &Path) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for d in fs::read(path).unwrap().lines() {
        map.push(d.unwrap().chars().collect());
    }

    map
}

fn find_antannaes(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Node>> {
    let mut antanaes: HashMap<char, Vec<Node>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != '.' {
                antanaes
                    .entry(*col)
                    .and_modify(|node_vec| {
                        node_vec.push(Node {freq: *col, location: (y as i32, x as i32)})
                    })
                    .or_insert(vec![Node { freq: *col, location: (y as i32, x as i32) }]);
            }
        }
    }

    antanaes

}

struct Node {
    freq: char,
    location: (i32, i32)
}

impl Node {
    pub fn distance(&self, other: &Node) -> i32 {
        (self.location.0 - other.location.0).abs() + (self.location.1 - other.location.1).abs()
    }

    pub fn line(&self, other: &Node) -> (i32, i32) {
        let unsimple = (self.location.0 - other.location.0, self.location.1 - other.location.1);
        let denom = gcd(unsimple.0, unsimple.1);

        (unsimple.0 / denom, unsimple.1 / denom)
    }

    pub fn find_antinodes(&self, other: &Node, map_size: (i32, i32)) -> HashSet<(i32, i32)> {
        if self.location == other.location {
            return HashSet::new();
        }
        let mut antinodes: HashSet<(i32,i32)> = HashSet::new(); 
        let line = self.line(other);

        for l in [line, (-line.0, -line.1)].iter() {

            let mut tc = Node { freq: ' ', location: self.location };
            while tc.location.0 < map_size.0 && tc.location.0 >=0 && tc.location.1 < map_size.1 && tc.location.1 >= 0 {
                antinodes.insert(tc.location);
                tc.location = (tc.location.0 + l.0, tc.location.1 + l.1);
            }
        }

        antinodes
    }
}
