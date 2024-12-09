use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::HashMap;
use std::cmp::Ordering;


fn analyze_data(data: String) -> (i32, i32) {
    // loop through each line and create an indexed array of all letters
    let (rules, reports) = data
        .split_once("\n\n")
        .map(|(rules, reports)| (
            rules
                .split("\n")
                .map(|rule|
                    rule
                        .split_once("|")
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                        .unwrap()
                ).fold(HashMap::new(), |mut acc: HashMap<i32, Vec<i32>>, r| {
                    acc.entry(r.0).and_modify(|e| e.push(r.1)).or_insert([r.1].to_vec());
                    acc
                }),
            reports
                .split("\n")
                .map(|report|
                    report
                        .split(",")
                        .map(|page| page.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                )
        ))
        .unwrap();

    let (correct, mut incorrect): (Vec<Vec<i32>>, Vec<Vec<i32>>) = reports.clone().partition(|report| {
        for (i, page) in report.iter().enumerate() {
            if let Some(rules) = rules.get(page) {
                if report[0..i].iter().any(|&page| rules.contains(&page)) {
                    return false;
                }
            }
        }
        true
    });

    let corrected = incorrect.into_iter().map(|mut report| {
        report.sort_by(|a, b| {
            let a_rules = rules.get(a);
            if a_rules.is_some_and(|rule| rule.contains(b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        return report;
    });


    return (
        correct.into_iter().map(|report| report[report.len() / 2]).sum(),
        corrected.into_iter().map(|report| report[report.len() / 2]).sum()
    );
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
        println!("Result Q1: {}\nResult Q2: {}", result.0, result.1);
    }

    #[test]
    fn test_example() {
        let data = String::from("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        let result = analyze_data(data);
        assert_eq!(result, (143, 123));
    }
}
