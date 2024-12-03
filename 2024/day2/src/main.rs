use std::fs;
use std::str::Split;
use std::path::PathBuf;

fn is_safe(prev: i32, num: i32, is_increasing: bool) -> bool {
    if is_increasing && prev > num {
        return false;
    } else if !is_increasing && prev < num {
        return false;
    }

    if (num - prev).abs() > 3 {
        return false;
    }
    if num == prev {
        return false;
    }
    return true;
}

fn is_report_safe(report: Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }
    let mut is_increasing = true;
    for (pos, level) in report.iter().enumerate() {
        if pos == 0 {
            continue;
        }
        if pos == 1 {
            if &report[pos - 1] > level {
                is_increasing = false;
            }
        }
        if !is_safe(report[pos - 1], *level, is_increasing) {
            return false;
        }
    }
    return true;
}

fn try_to_make_safe(report: Vec<i32>) -> bool {
    for (pos, _item) in report.iter().enumerate() {
        let mut new_report = report.clone();
        new_report.remove(pos);
        if is_report_safe(new_report) {
            return true;
        }
    }
    return false;
}

fn analyze_data(data: String) -> (i32, i32) {
    let lines: Split<'_, &str> = data.split("\n");
    let lin_count = lines.clone().count();
    let reports: Vec<Vec<i32>> = lines.map(|x| {
        let nums = x.split(" ");
        return nums.map(|y| {
            return y.parse::<i32>().unwrap();
        }).collect();
    }).collect();

    let mut unsafe_tally = 0;
    let mut xtra_unsafe_tally = 0;
    for report in reports {
        let mut safe = is_report_safe(report.clone());
        if safe {
            continue;
        }

        unsafe_tally += 1;

        safe = try_to_make_safe(report.clone());
        if !safe {
            xtra_unsafe_tally += 1;
        }
    }

    return ((lin_count - unsafe_tally).try_into().unwrap(), (lin_count - xtra_unsafe_tally).try_into().unwrap());
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
    fn test_is_safe() {
        assert_eq!(is_safe(1, 2, true), true);
        assert_eq!(is_safe(2, 1, true), false);
        assert_eq!(is_safe(1, 2, false), false);
        assert_eq!(is_safe(2, 1, false), true);
        assert_eq!(is_safe(1, 1, false), false);
        assert_eq!(is_safe(1, 1, true), false);
        assert_eq!(is_safe(5, 1, false), false);
        assert_eq!(is_safe(1, 5, false), false);
    }

    #[test]
    fn test_is_report_safe() {
        assert_eq!(is_report_safe(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_report_safe(vec![5, 4, 3, 2, 1]), true);
        assert_eq!(is_report_safe(vec![6, 1, 2, 3, 4, 5]), false);
        assert_eq!(is_report_safe(vec![1, 2, 3, 4, 5, 1]), false);
        assert_eq!(is_report_safe(vec![1, 2, 3, 4, 15]), false);
        assert_eq!(is_report_safe(vec![35, 31, 27, 26, 25, 23, 20, 19]), false);

        assert_eq!(is_report_safe(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_report_safe(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_report_safe(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_report_safe(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_report_safe(vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_report_safe(vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_try_to_make_safe() {
        assert_eq!(try_to_make_safe(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(try_to_make_safe(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(try_to_make_safe(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(try_to_make_safe(vec![1, 3, 2, 4, 5]), true);
        assert_eq!(try_to_make_safe(vec![8, 6, 4, 4, 1]), true);
        assert_eq!(try_to_make_safe(vec![1, 3, 6, 7, 9]), true);
    }
}
