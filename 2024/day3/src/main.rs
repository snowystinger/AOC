use std::fs;
use std::path::PathBuf;
use regex::{Regex, RegexSet};

fn analyze_data(data: &str) -> (i32, i32) {
    let patterns = [r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)", r"do\(\)", r"don't\(\)"];
    // Compile a set matching any of our patterns.
    let set = RegexSet::new(patterns).unwrap();
    // Compile each pattern independently.
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    let mut do_math = true;
    let mut real_order = vec![];
    let mut q1_final_result = 0;
    let mut q2_final_result = 0;
    set
        .matches(data)
        .into_iter()
        // Dereference the match index to get the corresponding
        // compiled pattern.
        .map(|index| &regexes[index])
        // To get match locations or any other info, we then have to search the
        // exact same haystack again, using our separately-compiled pattern.
        .for_each(|re| {
            for mat in re.captures_iter(data) {
                // println!("{:?}", mat);
                real_order.push(mat);
            }
        });
    real_order.sort_by(|a, b| a.get(0).unwrap().start().cmp(&b.get(0).unwrap().start()));
    real_order.iter().for_each(|c| {
        let first = c.get(0).unwrap().as_str();
        let pos = c.get(0).unwrap().start();
        println!("{} {}", pos, first);
        if first == "do()" {
            do_math = true;
            return;
        } else if first == "don't()" {
            do_math = false;
            return;
        } else if do_math {
            let first = c.get(1).unwrap().as_str();
            let second = c.get(2).unwrap().as_str();
            let num1 = first.parse::<i32>().unwrap();
            let num2 = second.parse::<i32>().unwrap();
            q1_final_result += num1 * num2;
            q2_final_result += num1 * num2;
        }
        if !do_math {
            let first = c.get(1).unwrap().as_str();
            let second = c.get(2).unwrap().as_str();
            let num1 = first.parse::<i32>().unwrap();
            let num2 = second.parse::<i32>().unwrap();
            q1_final_result += num1 * num2;
        }
    });

    return (q1_final_result, q2_final_result);
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
        let result = analyze_data(contents.as_str());
        println!("Result Q1: {}\nResult Q2 {}", result.0, result.1);
    }

}
