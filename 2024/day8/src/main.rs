use std::{fs, vec};
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::{thread, time};
use std::iter;
use itertools::{Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
    fn get_antinodes(&self, other: &Pos) -> Vec<Pos> {
        let distance = ((self.x - other.x), (self.y - other.y));
        let pos1 = Pos::new(self.x + distance.0, self.y + distance.1);
        let pos2 = Pos::new(self.x - distance.0, self.y - distance.1);
        let pos3 = Pos::new(other.x + distance.0, other.y + distance.1);
        let pos4 = Pos::new(other.x - distance.0, other.y - distance.1);
        if pos1.x == other.x && pos1.y == other.y {
            return vec![pos2, pos3];
        } else {
            return vec![pos1, pos4];
        }
    }
    fn get_antinodes_2(&self, other: &Pos, maxX: i32, maxY: i32) -> Vec<Pos> {
        let distance = ((self.x - other.x), (self.y - other.y));
        let mut positions = vec![];
        let mut nextX = self.x;
        let mut nextY = self.y;

        let pos = Pos::new(nextX, nextY);
        positions.push(pos);

        while nextX >= 0 && nextX < maxX && nextY >= 0 && nextY < maxY {
            nextX += distance.0;
            nextY += distance.1;
            let pos = Pos::new(nextX, nextY);
            positions.push(pos);
        }
        nextX = self.x;
        nextY = self.y;
        while nextX >= 0 && nextX < maxX && nextY >= 0 && nextY < maxY {
            nextX -= distance.0;
            nextY -= distance.1;
            let pos = Pos::new(nextX, nextY);
            positions.push(pos);
        }
        return positions;
    }
}

fn analyze_data(data: String) -> (i64, i64) {
    let maxX = data.lines().map(|line| line.len()).max().unwrap() as i32;
    let maxY = data.lines().count() as i32;
    let nodes = data
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, line)| {
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    let pos = Pos { x: x as i32, y: y as i32 };
                    acc.entry(char).and_modify(|value: &mut Vec<Pos>| value.push(pos)).or_insert(vec![pos]);
                }
            }
            acc
        });

    let q1_antinodes = nodes.clone()
        .iter()
        .map(|(key, positions)| {
            let combos = positions.iter().combinations(2);
            let antinodes = combos.flat_map(|combo| {
                let a = combo[0];
                let b = combo[1];
                return a.get_antinodes(b);
            });
            return antinodes.collect::<Vec<Pos>>();
        }).collect::<Vec<Vec<Pos>>>();

    let q2_antinodes = nodes.clone()
        .iter()
        .map(|(key, positions)| {
            let combos = positions.iter().combinations(2);
            let antinodes = combos.flat_map(|combo| {
                let a = combo[0];
                let b = combo[1];
                return a.get_antinodes_2(b, maxX, maxY);
            });
            return antinodes.collect::<Vec<Pos>>();
        }).collect::<Vec<Vec<Pos>>>();

    let q1_count =  q1_antinodes
        .iter()
        .fold(HashSet::<Pos>::new(), |mut acc, antinode_vec| {
            let antinodes = antinode_vec
                .iter()
                .filter(|pos| pos.x >= 0 && pos.x < maxX && pos.y >= 0 && pos.y < maxY);
            acc.extend(antinodes);
            acc
        }).len() as i64;

    let q2_count =  q2_antinodes
        .iter()
        .fold(HashSet::<Pos>::new(), |mut acc, antinode_vec| {
            let antinodes = antinode_vec
                .iter()
                .filter(|pos| pos.x >= 0 && pos.x < maxX && pos.y >= 0 && pos.y < maxY);
            acc.extend(antinodes);
            acc
        }).len() as i64;

    return (q1_count, q2_count);
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
        assert_eq!(result, 295);
    }

    #[test]
    fn test_example_1() {
        let data = String::from(
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............");
        let (result1, result2) = analyze_data(data.clone());
        assert_eq!(result1, 14);
        assert_eq!(result2, 34);
    }

    #[test]
    fn test_example_2() {
        let data = String::from(
"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........");
        let (result, result2) = analyze_data(data.clone());
        assert_eq!(result, 2);
        assert_eq!(result2, 5);
    }

    #[test]
    fn test_example_3() {
        let data = String::from(
"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........");
        let (result, result2) = analyze_data(data.clone());
        assert_eq!(result, 4);
        assert_eq!(result2, 8);
    }

    #[test]
    fn test_example_4() {
        let data = String::from(
"..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........");
        let (result, _) = analyze_data(data.clone());
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_5() {
        let data = String::from(
"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........");
        let (_, result) = analyze_data(data.clone());
        assert_eq!(result, 9);
    }

    #[test]
    fn test_pos_1() {
        let pos1 = Pos { x: 0, y: 0 };
        let pos2 = Pos { x: 1, y: 1 };
        let results = pos1.get_antinodes(&pos2);
        assert_eq!(results[0], Pos { x: -1, y: -1 });
        assert_eq!(results[1], Pos { x: 2, y: 2 });
    }

    #[test]
    fn test_pos_2() {
        let pos1 = Pos { x: 1, y: 1 };
        let pos2 = Pos { x: 0, y: 0 };
        let results = pos1.get_antinodes(&pos2);
        assert_eq!(results[0], Pos { x: 2, y: 2 });
        assert_eq!(results[1], Pos { x: -1, y: -1 });
    }

    #[test]
    fn test_pos_3() {
        let pos1 = Pos { x: 0, y: 1 };
        let pos2 = Pos { x: 1, y: 0 };
        let results = pos1.get_antinodes(&pos2);
        assert_eq!(results[0], Pos { x: -1, y: 2 });
        assert_eq!(results[1], Pos { x: 2, y: -1 });
    }

    #[test]
    fn test_pos_4() {
        let pos1 = Pos { x: 1, y: 0 };
        let pos2 = Pos { x: 0, y: 1 };
        let results = pos1.get_antinodes(&pos2);
        assert_eq!(results[0], Pos { x: 2, y: -1 });
        assert_eq!(results[1], Pos { x: -1, y: 2 });
    }

    // #[test]
    // fn test_pos2_1() {
    //     let pos1 = Pos { x: 0, y: 0 };
    //     let pos2 = Pos { x: 1, y: 1 };
    //     let results = pos1.get_antinodes_2(&pos2, 4, 4);
    //     assert_eq!(results[0], Pos { x: -1, y: -1 });
    //     assert_eq!(results[1], Pos { x: 2, y: 2 });
    //     assert_eq!(results[2], Pos { x: 3, y: 3 });
    //     assert_eq!(results[3], Pos { x: 4, y: 4 });
    // }
}
