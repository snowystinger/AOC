use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::{thread, time};

fn get_e(matrix: &Vec<Vec<char>>, max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if (x + 1) < max_x {
        let found = matrix[y][x + 1] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x + 1, y)
        });
    }
    return (false, true, (x, y));
}

fn get_ne(matrix: &Vec<Vec<char>>, max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if (x + 1) < max_x && y > 0 as usize {
        let found = matrix[y - 1][x + 1] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x + 1, y - 1)
        });
    }
    return (false, true, (x, y));
}

fn get_n(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if y > 0 as usize {
        let found = matrix[y - 1][x] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x, y - 1)
        });
    }
    return (false, true, (x, y));
}

fn get_nw(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if x > 0 as usize && y > 0 as usize {
        let found = matrix[y - 1][x - 1] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x - 1, y - 1)
        });
    }
    return (false, true, (x, y));
}

fn get_w(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if x > 0 as usize {
        let found = matrix[y][x - 1] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x - 1, y)
        });
    }
    return (false, true, (x, y));
}

fn get_sw(matrix: &Vec<Vec<char>>, _max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if x > 0 as usize && (y + 1) < max_y {
        let found = matrix[y + 1][x - 1] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x - 1, y + 1)
        });
    }
    return (false, true, (x, y));
}

fn get_s(matrix: &Vec<Vec<char>>, _max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    if (y + 1) < max_y {
        let found = matrix[y + 1][x] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x, y + 1)
        });
    }
    return (false, true, (x, y));
}

fn get_se(matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    // println!("comparison {} {} {} {} {} {} {} {}", x + 1, max_x, (x + 1) < max_x, y + 1, max_y, (y + 1) < max_y, matrix[y + 1][x + 1], search);
    if (x + 1) < max_x && (y + 1) < max_y {
        let found = matrix[y + 1][x + 1] == search;
        return (found, false, match found {
            true => (x, y),
            false => (x + 1, y + 1)
        });
    }
    return (false, true, (x, y));
}

fn search_in_direction(direction: &str , matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, bool, (usize, usize)) {
    match direction {
        "e" => return get_e(matrix, max_x, max_y, x, y, search),
        "ne" => return get_ne(matrix, max_x, max_y, x, y, search),
        "n" => return get_n(matrix, max_x, max_y, x, y, search),
        "nw" => return get_nw(matrix, max_x, max_y, x, y, search),
        "w" => return get_w(matrix, max_x, max_y, x, y, search),
        "sw" => return get_sw(matrix, max_x, max_y, x, y, search),
        "s" => return get_s(matrix, max_x, max_y, x, y, search),
        "se" => return get_se(matrix, max_x, max_y, x, y, search),
        _ => return (false, false, (x, y)),
    }
}

fn translate_direction(direction: &char) -> char {
    match direction {
        'n' => '^',
        's' => 'v',
        'w' => '<',
        'e' => '>',
        _ => ' ',
    }
}

fn change_direction(direction: &char) -> char {
    match direction {
        'n' => 'e',
        'e' => 's',
        's' => 'w',
        'w' => 'n',
        _ => ' ',
    }
}

fn search_for_loop(matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, &start_direction: &char, block_x: usize, block_y: usize) -> bool {
    let mut c_direction = start_direction;
    let mut lines = matrix.clone();
    let mut visited = HashSet::new();
    lines[y][x] = translate_direction(&c_direction);

    let (mut obstacle, mut off_edge, (mut x, mut y)) = search_in_direction(&c_direction.to_string(), &lines, max_x, max_y, x, y, '#');
    visited.insert(format!("{direction},{x},{y}", direction = &c_direction, x = x, y = y));
    lines[y][x] = match c_direction {
        'n' => '|',
        's' => '|',
        'w' => '-',
        'e' => '-',
        _ => ' ',
    };

    // let timeout = time::Duration::from_millis(250);
    while !off_edge {
        (obstacle, off_edge, (x, y)) = search_in_direction(&c_direction.to_string(), &lines, max_x, max_y, x, y, '#');
        if !obstacle && !off_edge {
            if visited.contains(&format!("{direction},{x},{y}", direction = &c_direction, x = x, y = y)) {
                // thread::sleep(time::Duration::from_millis(2500));
                lines[block_y][block_x] = 'O';
                // print_matrix(&lines);
                return true;
            }
        }
        visited.insert(format!("{direction},{x},{y}", direction = &c_direction, x = x, y = y));
        if obstacle {
            c_direction = change_direction(&c_direction);
            if lines[y][x] != '^' {
                lines[y][x] = '+';
            }
        } else {
            if lines[y][x] != '^' {
                lines[y][x] = match c_direction {
                    'n' => '|',
                    's' => '|',
                    'w' => '-',
                    'e' => '-',
                    _ => ' ',
                };
            }
        }
        // print!("\r{}\n", lines.clone().into_iter().map(|e| e.into_iter().map(|c| {
        //     match c {
        //         '#' => "\x1b[96m#\x1b[0m".to_string(),
        //         _ => c.to_string()
        //     }
        // }).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
        // thread::sleep(timeout);
    }
    return false;
}

fn place_block_before_and_search(lines: &Vec<Vec<char>>, max_x: usize, max_y: usize, start_x: usize, start_y: usize,  x: usize, y: usize, &start_direction: &char, &c_dirction: &char, tried: &HashSet<String>) -> bool {
    let next_coord = get_next_coord(max_x, max_y, x, y, &c_dirction.to_string());
    let key = format!("{},{}", next_coord.0, next_coord.1);
    if tried.contains(&key) {
        return false;
    }

    if !(next_coord.0 == x && next_coord.1 == y && lines[next_coord.1][next_coord.0] != '#') {
        let mut changed_layout = lines.clone();
        changed_layout[next_coord.1][next_coord.0] = '#';
        return search_for_loop(&changed_layout, max_x, max_y, start_x, start_y, &start_direction, next_coord.0, next_coord.1);
    }
    return false;
}

fn data_to_matrix(data: String) -> Vec<Vec<char>> {
    return data.lines().map(|line| line.chars().collect()).collect();
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    println!("{}\n", matrix.clone().into_iter().map(|e| e.into_iter().map(|c| {
        match c {
            '#' => "\x1b[96m#\x1b[0m".to_string(),
            _ => c.to_string()
        }
    }).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
}

fn get_next_coord(max_x: usize, max_y: usize, x: usize, y: usize, direction: &str) -> (usize, usize) {
    match direction {
        "e" => return match x < max_x - 1 { true => (x + 1, y), false => (x, y) },
        "ne" => return match x < max_x - 1 && y > 0 { true => (x + 1, y - 1), false => (x, y) },
        "n" => return match y > 0 { true => (x, y - 1), false => (x, y) },
        "nw" => return match x > 0 && y > 0 { true => (x - 1, y - 1), false => (x, y) },
        "w" => return match x > 0 { true => (x - 1, y), false => (x, y) },
        "sw" => return match x > 0 && y < max_y - 1 { true => (x - 1, y + 1), false => (x, y) },
        "s" => return match y < max_y - 1 { true => (x, y + 1), false => (x, y) },
        "se" => return match x < max_x - 1 && y < max_y - 1 { true => (x + 1, y + 1), false => (x, y) },
        _ => return (x, y),
    }
}

fn analyze_data(data: String) -> (i32, i32) {
    let matrix = data_to_matrix(data);
    let mut lines = matrix.clone();
    let max_x = lines[0].len();
    let max_y = lines.len();

    let mut start = (0, 0);
    let mut direction = ' ';
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' || *c == 'v' || *c == '<' || *c == '>' {
                start = (x, y);
                direction = *c;
                break;
            }
        }
        if direction != ' ' {
            break;
        }
    }

    let mut c_direction = match direction {
        '^' => 'n',
        'v' => 's',
        '<' => 'w',
        '>' => 'e',
        _ => ' ',
    };
    let start_direction = c_direction;

    let mut hash_set = HashSet::new();
    let coord = (vec![start.0, start.1]).into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    hash_set.insert(coord.join(","));
    let mut loops = 0;
    let mut tried = HashSet::new();
    if place_block_before_and_search(&matrix, max_x, max_y, start.0, start.1, start.0, start.1, &start_direction, &start_direction, &tried) {
        loops += 1;
    }
    tried.insert(format!("{},{}", start.0, start.1));

    let (mut obstacle, mut off_edge, (mut x, mut y)) = search_in_direction(&c_direction.to_string(), &lines, max_x, max_y, start.0, start.1, '#');
    lines[y][x] = translate_direction(&c_direction);

    // let timeout = time::Duration::from_millis(50);
    while !off_edge {
        if place_block_before_and_search(&matrix, max_x, max_y, start.0, start.1, x, y, &start_direction, &c_direction, &tried) {
            loops += 1;
        }
        tried.insert(format!("{},{}", x, y));

        let coord = (vec![x, y]).into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        hash_set.insert(coord.join(","));
        (obstacle, off_edge, (x, y)) = search_in_direction(&c_direction.to_string(), &lines, max_x, max_y, x, y, '#');
        if obstacle {
            c_direction = change_direction(&c_direction);
        } else {
            lines[y][x] = translate_direction(&c_direction);
        }
        // print!("\r {}", lines.clone().into_iter().map(|e| e.into_iter().map(|c| {
        //     match c {
        //         '#' => "\x1b[96m#\x1b[0m".to_string(),
        //         _ => c.to_string()
        //     }
        // }).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
        // thread::sleep(timeout);
    }
    // println!("{:?}", hash_set);

    return (
        hash_set.len() as i32, loops
    );
}

fn main() {
    let contents = String::from(
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
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
        let result = analyze_data(contents);
        // println!("Result Q1: {}\nResult Q2: {}", result.0, result.1);
        assert_eq!(result, (5534, 2262));
    }

    #[test]
    fn test_example_1() {
        let data = String::from(
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        let result = analyze_data(data);
        assert_eq!(result, (41, 6));
    }

    #[test]
    fn test_example_2() {
        let data = String::from(
"..........
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        let result = analyze_data(data);
        assert_eq!(result, (7, 0));
    }

    #[test]
    fn test_example_3() {
        let data = String::from(
"....#.....
..........
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        let result = analyze_data(data);
        assert_eq!(result, (11, 0));
    }

    #[test]
    fn test_example_4() {
        let data = String::from(
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
..........
#.........
......#...");
        let result = analyze_data(data);
        assert_eq!(result, (18, 0));
    }

    #[test]
    fn test_example_5() {
        let data = String::from(
"....#.....
.........#
..........
..#.......
.......#..
..........
....^.....
........#.
#.........
......#...");
        let result = analyze_data(data);
        assert_eq!(result, (22, 1));
    }

    #[test]
    fn test_search_for_loop() {
        let data = String::from(
"....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.#^---+.
........#.
#.........
......#...");
        let result = search_for_loop(&data_to_matrix(data), 10, 10, 4, 6, &'n', 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_for_loop_2() {
        let data = String::from(
"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
......#.#.
#.........
......#...");
        let result = search_for_loop(&data_to_matrix(data), 10, 10, 4, 6, &'n', 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_for_loop_3() {
        let data = String::from(
"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----+##.
#+----+...
......#...");
        let result = search_for_loop(&data_to_matrix(data), 10, 10, 4, 6, &'n', 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_for_loop_4() {
        let data = String::from(
"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
..|...|.#.
##+---+...
......#...");
        let result = search_for_loop(&data_to_matrix(data), 10, 10, 4, 6, &'n', 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_for_loop_5() {
        let data = String::from(
"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
....|.|.#.
#..#+-+...
......#...");
        let result = search_for_loop(&data_to_matrix(data), 10, 10, 4, 6, &'n', 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_for_loop_6() {
        let data = String::from(
"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......##..");
        let result = search_for_loop(&data_to_matrix(data), 10, 10, 4, 6, &'n', 0, 0);
        assert_eq!(result, true);
    }
}
