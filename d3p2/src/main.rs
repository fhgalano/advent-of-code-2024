use std::fs;
use std::path::Path;


fn main() {
    let input_path = Path::new("input");
    let input = fs::read_to_string(input_path).unwrap();

    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
    let res = do_math(input);
    dbg!(res);
}


fn do_math(mut file: String) -> i32 {
    let mut muls: Vec<i32> = Vec::new();
    let mut do_o_dont = true;
    while let Some(loc) = file.find("mul(") {
        let end = loc + 3 + file[loc+3..file.len()].find(")").unwrap_or(loc+4);
        if let Some(p) = validate_input(&file[loc+4..end], 2) {
            if let Some(n) = do_or_dont(&mut file, loc) {
                do_o_dont = n;
            }
            if do_o_dont {
                muls.push(p[0] * p[1]);
            };
            file = file[0..loc-1].to_owned() + &file[end..];
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

fn do_or_dont(file: &mut String, location: usize) -> Option<bool> {
    let do_ = match file.find("do()") {
        Some(n) if n < location => n,
        _ => 0
    };
    let dont = match file.find("don't()") {
        Some(n) if n < location => n,
        _ => 0
    };

    if do_ == 0 && dont == 0 {
        return None;
    }
    else if do_ < dont && dont < location {
        file.replace_range(dont+1..dont+1, "x");
        dbg!(&file);
        return Some(false);
    }
    else if dont < do_ && do_ < location {
        file.replace_range(do_+1..do_+1, "x");
        dbg!(&file);
        return Some(true);
    }
    None
}
