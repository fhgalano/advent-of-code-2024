use std::fs;
use std::io::BufRead;
use std::path::Path;


fn main() {
    let input = Path::new("input");

    let mut safe_reports: i32 = 0;
    for line in fs::read(input).unwrap().lines() {
        let report: Vec<i32> = line.unwrap().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        match determine_report_safety(&report) {
            Safety::Safe => safe_reports += 1,
            Safety::Unsafe => continue
        }
    }
    dbg!(safe_reports);
}


fn determine_report_safety(report: &Vec<i32>) -> Safety {

    let initial_trend = match trend(report[0] - report[1]) {
        Some(n) => n,
        _ => return Safety::Unsafe
    };

    for i in 1..report.len() {
        match trend(report[i - 1] - report[i]) {
            Some(n) => {
                match n == initial_trend {
                    true => continue,
                    false => return Safety::Unsafe
                }
            },
            None => return Safety::Unsafe
        };
    }

    Safety::Safe
}


fn trend(v: i32) -> Option<Trend> {
    match v {
        n if 3 >= n && n > 0 => Some(Trend::Up),
        n if -3 <= n && n < 0 => Some(Trend::Down),
        _ => None
    }
}


enum Safety {
    Safe,
    Unsafe,
}

#[derive(PartialEq)]
enum Trend {
    Up,
    Down,
}
