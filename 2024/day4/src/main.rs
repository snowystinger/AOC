use std::fs;
use std::path::PathBuf;


fn get_e(matrix: &Vec<Vec<char>>, max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if (x + 1) < max_x {
        return (matrix[y][x + 1] == search, (x+1, y));
    }
    return (false, (x+1, y));
}

fn get_ne(matrix: &Vec<Vec<char>>, max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if (x + 1) < max_x && y > 0 as usize {
        return (matrix[y - 1][x + 1] == search, (x+1, y-1));
    }
    return (false, (x, y));
}

fn get_n(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if y > 0 as usize {
        return (matrix[y - 1][x] == search, (x, y-1));
    }
    return (false, (x, y));
}

fn get_nw(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if x > 0 as usize && y > 0 as usize {
        return (matrix[y - 1][x - 1] == search, (x-1, y-1));
    }
    return (false, (x, y));
}

fn get_w(matrix: &Vec<Vec<char>>, _max_x: usize, _max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if x > 0 as usize {
        return (matrix[y][x - 1] == search, (x-1, y));
    }
    return (false, (x, y));
}

fn get_sw(matrix: &Vec<Vec<char>>, _max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if x > 0 as usize && (y + 1) < max_y {
        return (matrix[y + 1][x - 1] == search, (x-1, y+1));
    }
    return (false, (x, y));
}

fn get_s(matrix: &Vec<Vec<char>>, _max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    if (y + 1) < max_y {
        return (matrix[y + 1][x] == search, (x, y+1));
    }
    return (false, (x, y));
}

fn get_se(matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)) {
    // println!("comparison {} {} {} {} {} {} {} {}", x + 1, max_x, (x + 1) < max_x, y + 1, max_y, (y + 1) < max_y, matrix[y + 1][x + 1], search);
    if (x + 1) < max_x && (y + 1) < max_y {
        return (matrix[y + 1][x + 1] == search, (x+1, y+1));
    }
    return (false, (x, y));
}

fn search(func: fn (matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize, search: char) -> (bool, (usize, usize)), matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize) -> bool {
    let (found_m, (new_x, new_y)) = func(matrix, max_x, max_y, x, y, 'M');
    if !found_m {
        return false;
    }
    let (found_a, (new_x2, new_y2)) = func(matrix, max_x, max_y, new_x, new_y, 'A');
    if !found_a {
        return false;
    }
    let (found_s, (_, _)) = func(matrix, max_x, max_y, new_x2, new_y2, 'S');
    if !found_s {
        return false;
    }
    return true;
}

fn search_in_direction(direction: &str , matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize) -> bool {
    match direction {
        "e" => return search(get_e, matrix, max_x, max_y, x, y),
        "ne" => return search(get_ne, matrix, max_x, max_y, x, y),
        "n" => return search(get_n, matrix, max_x, max_y, x, y),
        "nw" => return search(get_nw, matrix, max_x, max_y, x, y),
        "w" => return search(get_w, matrix, max_x, max_y, x, y),
        "sw" => return search(get_sw, matrix, max_x, max_y, x, y),
        "s" => return search(get_s, matrix, max_x, max_y, x, y),
        "se" => return search(get_se, matrix, max_x, max_y, x, y),
        _ => return false,
    }
}

fn analyze_data(data: String) -> i32 {
    // loop through each line and create an indexed array of all letters
    let lines: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let max_x = lines[0].len();
    let max_y = lines.len();
    let mut count = 0;

    for (y, line) in lines.clone().iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'X' {
                if search_in_direction("e", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("ne", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("n", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("nw", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("w", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("sw", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("s", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
                if search_in_direction("se", &lines, max_x, max_y, x, y) {
                    count += 1;
                }
            }
        }
    }

  return count;
}

fn find_mas(matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize, x: usize, y: usize) -> bool {
    let (loc_m_nw, (_, _)) = get_nw(matrix, max_x, max_y, x, y, 'M');
    let (loc_m_ne, (_, _)) = get_ne(matrix, max_x, max_y, x, y, 'M');
    let (loc_m_se, (_, _)) = get_se(matrix, max_x, max_y, x, y, 'M');
    let (loc_m_sw, (_, _)) = get_sw(matrix, max_x, max_y, x, y, 'M');
    let (loc_s_nw, (_, _)) = get_nw(matrix, max_x, max_y, x, y, 'S');
    let (loc_s_ne, (_, _)) = get_ne(matrix, max_x, max_y, x, y, 'S');
    let (loc_s_se, (_, _)) = get_se(matrix, max_x, max_y, x, y, 'S');
    let (loc_s_sw, (_, _)) = get_sw(matrix, max_x, max_y, x, y, 'S');

    let has_mas_nw_se = (loc_m_nw && loc_s_se) || (loc_s_nw && loc_m_se);
    let has_mas_ne_sw = (loc_m_ne && loc_s_sw) || (loc_s_ne && loc_m_sw);

    return has_mas_nw_se && has_mas_ne_sw;
}

fn q2_analyze_data(data: String) -> i32 {
    // loop through each line and create an indexed array of all letters
    let lines: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let max_x = lines[0].len();
    let max_y = lines.len();
    let mut count = 0;

    for (y, line) in lines.clone().iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'A' {
                if find_mas(&lines, max_x, max_y, x, y) {
                    count += 1;
                }
            }
        }
    }

    return count;
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
        println!("Result Q1: {}", result);
    }

    #[test]
    fn test_answer2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test-data/input.txt");
        let contents: String = fs::read_to_string(d).expect("Something went wrong reading the file");
        let result = q2_analyze_data(contents);
        println!("Result Q2: {}", result);
    }

    #[test]
    fn test_get_e() {
        let matrix = vec![vec!['X', 'M', 'A', 'S']];
        let (result, _) = get_e(&matrix, 4, 1, 0, 0, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_e(&matrix, 4, 1, 1, 0, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_e(&matrix, 4, 1, 2, 0, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_e_incomplete() {
        let matrix = vec![vec!['X', 'M', 'M', 'S']];
        let (result, _) = get_e(&matrix, 4, 1, 3, 0, 'A');
        assert_eq!(result, false);
        let (result_2, _) = get_e(&matrix, 4, 1, 3, 0, 'S');
        assert_eq!(result_2, false);
    }

    #[test]
    fn test_get_s() {
        let matrix = vec![vec!['X'], vec!['M'], vec!['A'], vec!['S']];
        let (result, _) = get_s(&matrix, 1, 4, 0, 0, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_s(&matrix, 1, 4, 0, 1, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_s(&matrix, 1, 4, 0, 2, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_w() {
        let matrix = vec![vec!['S', 'A', 'M', 'X']];
        let (result, _) = get_w(&matrix, 4, 1, 3, 0, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_w(&matrix, 4, 1, 2, 0, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_w(&matrix, 4, 1, 1, 0, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_w_incomplete() {
        let matrix = vec![vec!['X', 'M', 'M', 'S']];
        let (result, _) = get_w(&matrix, 4, 4, 0, 0, 'M');
        assert_eq!(result, false);
        let (result_2, _) = get_w(&matrix, 4, 4, 2, 0, 'A');
        assert_eq!(result_2, false);
    }

    #[test]
    fn test_get_n() {
        let matrix = vec![vec!['S'], vec!['A'], vec!['M'], vec!['X']];
        let (result, _) = get_n(&matrix, 1, 4, 0, 3, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_n(&matrix, 1, 4, 0, 2, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_n(&matrix, 1, 4, 0, 1, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_se() {
        let matrix = vec![vec!['X', ' ', ' ', ' '], vec![' ', 'M', ' ', ' '], vec![' ', ' ', 'A', ' '], vec![' ', ' ', ' ', 'S']];
        let (result, _) = get_se(&matrix, 4, 4, 0, 0, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_se(&matrix, 4, 4, 1, 1, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_se(&matrix, 4, 4, 2, 2, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_sw() {
        let matrix = vec![vec![' ', ' ', ' ', 'X'], vec![' ', ' ', 'M', ' '], vec![' ', 'A', ' ', ' '], vec!['S', ' ', ' ', ' ']];
        let (result, _) = get_sw(&matrix, 4, 4, 3, 0, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_sw(&matrix, 4, 4, 2, 1, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_sw(&matrix, 4, 4, 1, 2, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_nw() {
        let matrix = vec![vec!['S', ' ', ' ', ' '], vec![' ', 'A', ' ', ' '], vec![' ', ' ', 'M', ' '], vec![' ', ' ', ' ', 'X']];
        let (result, _) = get_nw(&matrix, 4, 4, 3, 3, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_nw(&matrix, 4, 4, 2, 2, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_nw(&matrix, 4, 4, 1, 1, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_get_ne() {
        let matrix = vec![vec![' ', ' ', ' ', 'S'], vec![' ', ' ', 'A', ' '], vec![' ', 'M', ' ', ' '], vec!['X', ' ', ' ', ' ']];
        let (result, _) = get_ne(&matrix, 4, 4, 0, 3, 'M');
        assert_eq!(result, true);
        let (result_2, _) = get_ne(&matrix, 4, 4, 1, 2, 'A');
        assert_eq!(result_2, true);
        let (result_3, _) = get_ne(&matrix, 4, 4, 2, 1, 'S');
        assert_eq!(result_3, true);
    }

    #[test]
    fn test_search_from_0_0() {
        let matrix = vec![vec!['X', ' ', ' ', ' '], vec![' ', 'M', ' ', ' '], vec![' ', ' ', 'A', ' '], vec![' ', ' ', ' ', 'S']];
        let result = search(get_se, &matrix, 4, 4, 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_in_direction_e_from_0_0() {
        let matrix = vec![vec!['X', 'M', 'A', 'S']];
        let result = search_in_direction("e", &matrix, 4, 1, 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_in_direction_e_from_0_0_with_extra_lines() {
        let matrix = vec![vec!['X', 'M', 'A', 'S'], vec![' ', ' ', ' ', ' '], vec![' ', ' ', ' ', ' '], vec![' ', ' ', ' ', ' ']];
        let result = search_in_direction("e", &matrix, 4, 4, 0, 0);
        assert_eq!(result, true);
    }

    #[test]
    fn test_single_e_w() {
        let contents = String::from("XMAS");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_n_s() {
        let contents = String::from("X\nM\nA\nS");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_w_e() {
        let contents = String::from("SAMX");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_s_n() {
        let contents = String::from("S\nA\nM\nX");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_nw_se() {
        let contents = String::from("X   \n M  \n  A \n   S");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_ne_sw() {
        let contents = String::from("   X\n  M \n A  \nS   ");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_se_nw() {
        let contents = String::from("   S\n  A \n M  \nX   ");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_sw_ne() {
        let contents = String::from("S   \n A  \n  M \n   X");
        let result = analyze_data(contents);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example() {
        let contents = String::from("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        let result = analyze_data(contents);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_example_2() {
        let contents = String::from(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........");
        let result = q2_analyze_data(contents);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_example_2_2() {
        let contents = String::from("M.S
.A.
M.S");
        let result = q2_analyze_data(contents);
        assert_eq!(result, 1);
    }

}
