use std::{fs, vec};
use std::path::PathBuf;


fn get_coord() {}

fn analyze_data(data: String) -> i32 {
    // loop through each line and create an indexed array of all letters
    let mut lines = data.lines();
    let mut i_lines = vec![];
    for line in lines {
        let char_string = line.chars();
        i_lines.push(char_string);
    }

    for line in i_lines {
        for char in line {
            print!("{}", char);
        }
    }

  return 0;
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

}
