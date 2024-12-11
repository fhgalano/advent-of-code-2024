use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::BufRead;
use std::path::Path;
use std::slice::Iter;
use itertools::Itertools;


fn main() {
    let values = parse_values(Path::new("value_input"));
    let rules = parse_rules(Path::new("rule_input"));

    let mut big_boi: i32 = 0;

    for l in values.iter() {
        let applicable_rules = gather_rules(l, &rules);

        match applicable_rules.iter().map(|x| apply_rule(l, *x)).all(|n| n) {
            true => big_boi += l[l.len() / 2],
            false => println!("bad {:?}", l)
        };
    }
    println!("Sup");
}

fn parse_rules(p: &Path) -> HashMap<i32, Vec<[i32;2]>> {
    let mut rules: HashMap<i32, Vec<[i32;2]>> = HashMap::new();
    for (v0, v1) in 
        read_to_string(p)
        .unwrap()
        .lines()
        .map(|x| x.split("|").map(|n| n.parse::<i32>().unwrap()).collect_tuple::<(i32,i32)>().unwrap())
        .collect::<Vec<_>>() {
        rules.entry(v0).and_modify(|l| l.push([v0, v1])).or_insert(vec![[v0, v1]]);
    }
    rules
}

fn parse_values(p: &Path) -> Vec<Vec<i32>> {
    read_to_string(p)
        .unwrap()
        .lines()
        .map(|x| x.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect()
}

fn gather_rules(target: &Vec<i32>, rules: &HashMap<i32, Vec<[i32;2]>>) -> Vec<[i32;2]> {
    let mut applicable_rules: Vec<[i32;2]> = Vec::new();

    for i in target.iter() {
        if let Some(known) = rules.get(i) {
            known.iter().for_each(|x| applicable_rules.push(*x));
        }
    }

    applicable_rules
}

fn apply_rule(target: &Vec<i32>, rule: [i32;2]) -> bool {
    match target.iter().position(|x| *x == rule[1]) {
        Some(rpos) => match target.iter().position(|x| *x == rule[0]) {
            Some(lpos) if lpos < rpos => true,
            _ => false
        }
        None => true
    }
}
