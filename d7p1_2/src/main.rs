use std::fs::read_to_string;
use std::io::BufReader;
use std::path::Path;


fn main() {
    let input = Path::new("input");

    let equations = get_equations(input);

    let mut total: i128 = 0;
    for eq in equations.iter() {
        if solveable(eq.answer, &eq.members, eq.members[0]) {
            total += eq.answer;
        }
    }
    dbg!(total);
}

fn get_equations(p: &Path) -> Vec<Equation> {
    let mut equations = Vec::new();
    for l in read_to_string(p).unwrap().lines() {
        let parts: Vec<&str> = l.split(':').map(|x| x.trim()).collect();
        let values: Vec<i128> = parts[1].split(' ').map(|x| x.parse::<i128>().unwrap()).collect();

        equations.push(
            Equation {
                answer: parts[0].parse::<i128>().unwrap(),
                members: values
            }
        )
    }

    equations
}

struct Equation {
    answer: i128,
    members: Vec<i128>,
}

enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    pub fn apply<T>(&self, a: T, b: T) -> T 
    where T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> 
    + std::string::ToString + std::fmt::Debug 
    +  std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => (a.to_string() + b.to_string().as_str()).parse::<T>().unwrap(),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Operator::Add,
            Operator::Multiply,
            Operator::Concat,
        ]
    }
}

fn solveable(answer: i128, vals: &[i128], total: i128) -> bool {
    if vals.len() == 1 {
        return total == answer 
    };

    for op in Operator::all().iter() {
        if solveable(answer, &vals[1..], op.apply(total, vals[1])) {
            return true;
        };
    };

    false
}

