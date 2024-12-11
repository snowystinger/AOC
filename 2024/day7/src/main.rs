use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::{thread, time};
use itertools::Itertools;
use std::iter;


fn analyze_data(data: String) -> i64 {
    let sums = data
        .lines()
        .flat_map(|line| {
            let findings = line.split_once(": ")
                .map(|(total, numbers)| {
                    let total = &total.parse::<i64>().unwrap();
                    let mut done = false;
                    let values = &numbers
                        .split(" ")
                        .map(|n| n.parse::<i64>().unwrap());
                    let results = iter::repeat_n(vec!['+', '*'], values.clone().count() - 1)
                        .multi_cartesian_product()
                        .filter_map(|operations| {
                            let mut values_copy = values.clone();
                            let mut result = values_copy.next().unwrap();
                            for operation in operations.clone() {
                                let next = values_copy.next().unwrap();
                                match operation {
                                    '+' => result += next,
                                    '*' => result *= next,
                                    _ => panic!("Unknown operation")
                                }
                            }
                            if result == *total && !done {
                                done = true;
                                return Some(result);
                            }
                            return None;
                        }).collect::<Vec<i64>>();
                    return results;
                }).unwrap();
            return findings;
        }).collect::<Vec<i64>>();

    return sums.iter().sum();
}


fn analyze_data_2(data: String) -> i64 {
    let sums = data
        .lines()
        .flat_map(|line| {
            let findings = line.split_once(": ")
                .map(|(total, numbers)| {
                    let total = &total.parse::<i64>().unwrap();
                    let mut done = false;
                    let values = &numbers
                        .split(" ")
                        .map(|n| n.parse::<i64>().unwrap());
                    let results = iter::repeat_n(vec!['+'.to_string(), '*'.to_string(), "||".to_string()], values.clone().count() - 1)
                        .multi_cartesian_product()
                        // could make this more efficient by just using a find instead of finding all combinations, but i read the question wrong initially
                        .filter_map(|operations| {
                            let mut values_copy = values.clone();
                            let mut result = values_copy.next().unwrap();
                            for operation in operations.clone() {
                                let next = values_copy.next().unwrap();
                                match operation.as_str() {
                                    "+" => result += next,
                                    "*" => result *= next,
                                    "||" => result = format!("{result}{next}", result=result, next=next).parse::<i64>().unwrap(),
                                    _ => panic!("Unknown operation")
                                }
                            }
                            if result == *total && !done {
                                done = true;
                                return Some(result);
                            }
                            return None;
                        }).collect::<Vec<i64>>();
                    return results;
                }).unwrap();
            return findings;
        }).collect::<Vec<i64>>();

    return sums.iter().sum();
}


fn main() {
    let contents = String::from(
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    );
    analyze_data(contents); // turn on the animations above for debugging
    // println!("Result Q1: {}\nResult Q2: {}", result.0, result.1);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_answer() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test-data/input.txt");
        let contents: String = fs::read_to_string(d).expect("Something went wrong reading the file");
        let result = analyze_data(contents.clone());
        let result2 = analyze_data_2(contents.clone());
        println!("Result Q1: {}\nResult Q2: {}", result, result2);
        assert_eq!(result, 2299996598890);
        assert_eq!(result2, 0362646859298554);
    }

    #[test]
    fn test_example_1() {
        let data = String::from(
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20");
        let result = analyze_data(data.clone());
        let result2 = analyze_data_2(data.clone());
        assert_eq!(result, 3749);
        assert_eq!(result2, 11387);
    }
}
