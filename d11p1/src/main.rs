use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let mut stones = get_stones_from_file(Path::new("test_input"));

    for _i in 0..75 {
        blink(&mut stones);
        println!("{:?}", _i);
    }

    dbg!(stones.len());
}

fn blink(stones:&mut Vec<i64>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = Vec::new();
    for i in stones.iter() {
        match i {
            0 => new_stones.push(1),
            n => {
                let str_n = n.to_string();
                match str_n.len() % 2 == 0 {
                    true => {
                        let (s1, s2) = str_n.split_at(str_n.len() /2 );
                        new_stones.push(s1.parse().unwrap());
                        new_stones.push(s2.parse().unwrap());
                    },
                    false => new_stones.push(n * 2024),
                };
            }
        };
    };
    new_stones
}

fn get_stones_from_file(p: &Path) -> Vec<i64> {
    read_to_string(p)
        .unwrap()
        .trim()
        .split(|x| x == ' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}
