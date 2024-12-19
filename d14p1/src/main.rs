use std::fmt;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::read;
use std::io::BufRead;
use std::path::Path;

use itertools::Itertools;

fn main() {
    let mut bathroom_map = Field::new(
        103,
        101,
        Some(get_robots_from_file(Path::new("input"))),
        //7,
        //11,
        //Some(get_robots_from_file(Path::new("test_input"))),
    ).unwrap();

    bathroom_map.move_robots(100);
    let map = bathroom_map.eval_quadrants();
    dbg!(map.values().product::<i64>());
}

struct Robot {
    location: (i64, i64),
    velocity: (i64, i64),
    id: i64,
}

struct Field {
    height: i64,
    width: i64,
    robots: Vec<Robot>,
}

impl Field {
    pub fn move_robots(&mut self, seconds: i64) {
        for rob in self.robots.iter_mut() {
            let total_movement = (rob.location.0 + rob.velocity.0 * seconds, rob.location.1 + rob.velocity.1 * seconds);
            rob.location = (
                match total_movement.0 < 0 {
                    true => match total_movement.0.abs() % self.width {
                        0 => 0,
                        n =>self.width - n,
                    },
                    false => total_movement.0 % self.width
                },
                match total_movement.1 < 0 {
                    true => match total_movement.1.abs() % self.height { 
                        0 => 0,
                        n => self.height - n,
                    },
                    false => total_movement.1 % self.height
                },
            );
        }
    }

    pub fn new(height: i64, width: i64, robots: Option<Vec<Robot>>) -> Result<Self, AnError> {
        for i in [height, width].iter() {
            if i % 2 != 1 {
                return Err(AnError::new("Fields cannot have any even height or width"))
            }
        };

        Ok(Self {
            height,
            width,
            robots: robots.unwrap_or_default(), // returns whatever is implemented for the Default trait
        })
    }
    pub fn eval_quadrants(&self) -> HashMap<&str, i64> {
        let mut quadrants = HashMap::new();
        let height_m = self.height / 2;
        let width_m  = self.width / 2;
        for rob in self.robots.iter() {
            if rob.location.0 != width_m && rob.location.1 != height_m {
                match (
                    rob.location.0 < width_m,
                    rob.location.1 < height_m
                ) {
                    (true, true) => quadrants.entry("q1").and_modify(|counter| *counter += 1).or_insert(1),
                    (false, true) => quadrants.entry("q2").and_modify(|counter| *counter += 1).or_insert(1),
                    (true, false) => quadrants.entry("q3").and_modify(|counter| *counter += 1).or_insert(1),
                    (false, false) => quadrants.entry("q4").and_modify(|counter| *counter += 1).or_insert(1),
                };
            }
        };
        quadrants
    }
}

fn get_robots_from_file(p: &Path) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    for (id_counter, instruction) in read(p).unwrap().lines().enumerate() {
        let line = instruction.unwrap();
        let pos_vel: Vec<&str> = line.split(' ').collect();
        let pos: (i64, i64) = pos_vel[0].trim_start_matches("p=").split(',').map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap();
        let vel: (i64, i64) = pos_vel[1].trim_start_matches("v=").split(',').map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap();

        robots.push(Robot { location: pos, velocity: vel, id: id_counter as i64 });
    }
    
    robots
}

#[derive(Debug)]
struct AnError {
    msg: String
}

impl AnError {
    pub fn new(msg: &str) -> Self {
        AnError {
            msg: msg.to_string()
        }
    }
}

impl fmt::Display for AnError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for AnError {}
