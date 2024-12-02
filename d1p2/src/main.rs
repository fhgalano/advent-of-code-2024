use std::fs;
use std::collections::HashMap;
use std::io::BufRead;
use std::iter::zip;
use std::path::Path;

fn main() {
    let input_path = Path::new("input");
    
    let mut l1: Vec<i32> = Vec::new();
    let mut l2 = HashMap::new();

    for d in fs::read(input_path).unwrap().lines() {
        let r: Vec<i32> = d.unwrap().split("   ").map(|x| x.parse().unwrap()).collect();

        l1.push(r[0]);
        l2.entry(r[1]).and_modify(|occ| *occ += 1).or_insert(1);
    }

    let mut total_similarity: i32 = 0;

    for v in l1.iter() {
        let similarity = v * l2.get(v).unwrap_or(&0);
        total_similarity += similarity;
    }

    dbg!(total_similarity);
}
