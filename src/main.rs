mod day1;

fn main() {
    day_1();
}

fn day_1() {
    println!("Day 1 -----------");
    let day1_input = day1::input(String::from("input/day1.txt"));
    println!("P1: {}", day1::day1_p1(day1_input));
    let day1_input = day1::input(String::from("input/day1.txt"));
    println!("P2: {}", day1::day1_p2(day1_input));
    println!("-----------------");
}


