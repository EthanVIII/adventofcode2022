pub fn input(filepath: String) -> Vec<Vec<i32>> {
    let contents = std::fs::read_to_string(filepath)
        .expect("[ERROR] Should have been able to read file");
    let mut return_vec: Vec<Vec<i32>> = Vec::new();
    let mut build_vec: Vec<i32> = Vec::new();
    let split_input: Vec<&str> = contents.split("\n").collect();
    for i in split_input {
        if i == "\r" && build_vec.len() > 0 {
            return_vec.push(build_vec);
            build_vec = Vec::new()
        }
        else {
            let v: i32 = String::from(i).replace("\r","").parse().unwrap();
            build_vec.push(v);
        }
    }
    if return_vec.len() > 0 {
        return_vec.push(build_vec);
    }
    return_vec
}

pub fn day1_p1(input: Vec<Vec<i32>>) -> i32 {
    let x: Vec<i32> = input
        .iter()
        .map(|c: &Vec<i32>| c.iter().sum())
        .collect();
    let y: i32 = *x.iter().max().unwrap();
    return y;
}

pub fn day1_p2(input:Vec<Vec<i32>>) -> i32 {
    let mut x: Vec<i32> = input
        .iter()
        .map(|c: &Vec<i32>| c.iter().sum())
        .collect();
    x.sort();
    x.reverse();
    return x[0..3].iter().sum();
}