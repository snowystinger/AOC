use std::fs;
use std::str::Split;

fn main() {
    let contents: String = fs::read_to_string("./tests/input.txt").expect("Something went wrong reading the file");
    // [[x, y], [x, y], ...]
    let lines: Split<'_, &str> = contents.split("\n");
    let lin_count = lines.clone().count();
    let pairs: Vec<Vec<i32>> = lines.map(|x| {
        let nums = x.split("   ");
        return nums.map(|y| {
            return y.parse::<i32>().unwrap();
        }).collect();
    }).collect();
    let mut first:Vec<i32> = Vec::with_capacity(lin_count);
    let mut second:Vec<i32> = Vec::with_capacity(lin_count);
    for pair in pairs {
        first.push(pair[0]);
        second.push(pair[1]);
    }
    first.sort();
    second.sort();
    let result = first.iter().zip(second.iter()).map(|(x, y)| {
        return (x - y).abs();
    }).sum::<i32>();
    println!("Result Q1: {}", result);

    // Q2
    let mut dict = std::collections::HashMap::new();
    for item in second {
        let count = dict.entry(item).or_insert(0);
        *count += 1;
    }
    let mut result = 0;
    for item in first {
        result += item * dict.get(&item).cloned().unwrap_or(0);
    }
    println!("Result Q2: {}", result);
}
