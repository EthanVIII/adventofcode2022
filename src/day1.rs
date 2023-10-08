pub fn input(filepath: String) -> Vec<Vec<String>> {
    let contents = std::fs::read_to_string(filepath)
        .expect("[ERROR] Should have been able to read file");
    let mut return_vec: Vec<Vec<String>> = Vec::new();
    let mut build_vec: Vec<String> = Vec::new();
    let split_input: Vec<&str> = contents.split("\n").collect();
    for i in split_input {
        if i == "\r" && build_vec.len() > 0 {
            return_vec.push(build_vec);
            build_vec = Vec::new()
        }
        else {
            build_vec.push(String::from(i));
        }
    }
    if return_vec.len() > 0 {
        return_vec.push(build_vec);
    }
    return_vec
}
