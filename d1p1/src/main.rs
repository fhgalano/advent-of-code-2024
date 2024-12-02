use std::fs;
use std::io::prelude;
use std::io::BufRead;
use std::iter::zip;
use std::path::Path;

fn main() {
    let input_path = Path::new("input");
    
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    for d in fs::read(input_path).unwrap().lines() {
        let r: Vec<i32> = d.unwrap().split("   ").map(|x| x.parse().unwrap()).collect();

        l1.push(r[0]);
        l2.push(r[1]);
    }

    l1.sort();
    l2.sort();

    let mut total_distance: i32 = 0;

    for (vl, vr) in zip(l1, l2) {
        let distance = vl - vr;
        total_distance += distance.abs();
    }

    dbg!(total_distance);
}
