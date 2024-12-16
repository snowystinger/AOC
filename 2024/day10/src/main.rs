use std::fmt::format;
use std::{fs, vec};
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::{thread, time};
use std::iter;


fn get_e(matrix: &Vec<Vec<char>>, max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if (x + 1) < max_x {
        let found = matrix[y][x + 1] == search;
        return (found, false, match found {
            true => (x + 1, y),
            false => (x + 1, y)
        });
    }
    return (false, true, (x, y));
}

fn get_n(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if y > 0 as usize {
        let found = matrix[y - 1][x] == search;
        return (found, false, match found {
            true => (x, y - 1),
            false => (x, y - 1)
        });
    }
    return (false, true, (x, y));
}

fn get_w(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if x > 0 as usize {
        let found = matrix[y][x - 1] == search;
        return (found, false, match found {
            true => (x - 1, y),
            false => (x - 1, y)
        });
    }
    return (false, true, (x, y));
}

fn get_s(matrix: &Vec<Vec<char>>, _max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if (y + 1) < max_y {
        let found = matrix[y + 1][x] == search;
        return (found, false, match found {
            true => (x, y + 1),
            false => (x, y + 1)
        });
    }
    return (false, true, (x, y));
}

fn search_in_direction(direction: &str , matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    match direction {
        "e" => return get_e(matrix, max_x, max_y, x, y, search),
        "n" => return get_n(matrix, max_x, max_y, x, y, search),
        "w" => return get_w(matrix, max_x, max_y, x, y, search),
        "s" => return get_s(matrix, max_x, max_y, x, y, search),
        _ => return (false, false, (x, y)),
    }
}

fn find_trail(matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, trail: Vec<char>, reachable: &mut HashMap<Coord, i32>) {
    let mut next_trail = trail.clone();
    let trail_char = next_trail.pop().unwrap();
    for direction in ["e", "n", "w", "s"].iter() {
        let (found, out_of_bounds, new_coord) = search_in_direction(&direction, matrix, max_x, max_y, x, y, trail_char);
        if found && trail_char == '9' && !out_of_bounds {
            *reachable.entry(Coord {x: new_coord.0, y: new_coord.1}).or_insert(0) += 1;
            continue;
        }
        if found && !out_of_bounds {
            find_trail(matrix, max_x, max_y, new_coord.0, new_coord.1, next_trail.clone(), reachable);
            continue;
        }
        if out_of_bounds {
            continue;
        }
    }
}

fn data_to_matrix(data: String) -> Vec<Vec<char>> {
    return data.lines().map(|line| line.chars().collect()).collect();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn analyze_data(data: String) -> (i64, i64) {
    let matrix = data_to_matrix(data);
    let mut lines = matrix.clone();
    let max_x = lines[0].len();
    let max_y = lines.len();

    let trail_heads = matrix.iter().enumerate().fold(Vec::new(), |mut acc, (y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c == '0' {
                acc.push(Coord {x: x, y: y});
            }
        });
        acc
    });

    let trail = vec!['9', '8', '7', '6', '5', '4', '3', '2', '1'];
    let mut sum = 0;
    let mut sum2 = 0;
    for trail_head in trail_heads {
        let mut reachable: HashMap<Coord, i32> = HashMap::new();
        find_trail(&matrix, max_x, max_y, trail_head.x, trail_head.y, trail.clone(), &mut reachable);
        sum += reachable.iter().count() as i64;
        sum2 += reachable.iter().fold(0, |mut acc, (_, c)| {
            acc += c;
            acc
        });
    }

    return (sum, sum2 as i64);
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
        let (result, result2) = analyze_data(contents.clone());
        println!("Result Q1: {}", result);
        println!("Result Q2: {}", result2);
        assert_eq!(result, 841);
        assert_eq!(result2, 1875);
    }

    #[test]
    fn test_example_1() {
        let data = String::from(
"0123
1234
8765
9876");
        let (result1, _) = analyze_data(data.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn test_example_2() {
        let data = String::from(
"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9");
        let (result1, result2) = analyze_data(data.clone());
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn test_example_3() {
        let data = String::from(
"..90..9
...1.98
...2..7
6543456
765.987
876....
987....");
        let (result1, _) = analyze_data(data.clone());
        assert_eq!(result1, 4);
    }

    #[test]
    fn test_example_4() {
        let data = String::from(
"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01");
        let (result1, _) = analyze_data(data.clone());
        assert_eq!(result1, 3);
    }
}
