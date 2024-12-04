use std::fs;
use std::io::BufRead;
use std::marker::PhantomData;
use std::path::Path;


fn main() {
    let input_path = Path::new("input");
    let input = fs::read_to_string(input_path).unwrap();

    let res = do_math(input);
    dbg!(res);
}


fn do_math(mut file: String) -> i32 {
    let mut muls: Vec<i32> = Vec::new();
    while let Some(loc) = file.find("mul(") {
        dbg!(loc);
        dbg!(&file[loc+3..]);
        let end = loc + 3 + file[loc+3..file.len()].find(")").unwrap_or(loc+4);
        dbg!(&end);
        if let Some(p) = validate_input(&file[loc+4..end], 2) {
            dbg!(&p);
            muls.push(p[0] * p[1]);
            file = file[0..loc-1].to_owned() + &file[end..];
            dbg!(&file);
        }
        else {
            file.remove(loc);
        }
    }
    return muls.iter().sum();
}


fn validate_input(input: &str, num_args: u8) -> Option<Vec<i32>> {
    let params: Vec<&str> = input.split(',').collect();

    if params.len() as u8 != num_args {
        return None;
    }
    
    let mut parsed_params: Vec<i32> = Vec::new();
    for val in params.into_iter() {
        match val.parse::<i32>() {
            Ok(n) => parsed_params.push(n),
            _ => return None
        }
    };

    Some(parsed_params)
}
