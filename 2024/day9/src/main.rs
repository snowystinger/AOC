use std::fmt::format;
use std::{fs, vec};
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::{thread, time};
use std::iter;



#[derive(Debug, Clone)]
struct IdGenerator {
    id: i32
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator { id: -1 }
    }
    fn create_file(&mut self, position: i32, size: i32) -> File {
        self.id += 1;
        return File::new(self.id, position, size);
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct File {
    id: i32,
    position: i32,
    size: i32
}

impl File {
    fn new(id: i32, position: i32, size: i32) -> File {
        File { id, position, size }
    }
}

fn check_sum(fs: FileSystem) -> i64 {
    let mut sum: i64 = 0;
    for file in fs.files {
        if file.id == -1 {
            continue;
        }
        for i in file.position..(file.position + file.size) {
            sum += i as i64 * file.id as i64;
        }
    }
    return sum;
}

#[derive(Debug, Clone)]
struct FileSystem {
    files: Vec<File>,
    id_generator: IdGenerator,
    total_used: i32
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem { files: vec![], id_generator: IdGenerator::new(), total_used: 0 }
    }

    fn add_file(&mut self, position: i32, size: i32) {
        let file = self.id_generator.create_file(position, size);
        self.files.push(file);
        self.total_used += size;
    }
    fn add_freespace(&mut self, position: i32, size: i32) {
        let file = File { id: -1, position, size };
        self.files.push(file);
    }
    fn compact(&mut self) -> FileSystem {
        let mut c_fs = FileSystem::new();
        let mut reverse = self.files.clone().into_iter().filter(|f| f.id != -1).collect::<Vec<File>>();
        reverse.reverse();
        let mut reversed = reverse.into_iter();
        let mut borrow_file = reversed.next().unwrap();
        let mut total_used = 0;
        self.files.clone().into_iter().for_each(|(file)| {
            if self.total_used <= total_used {
                return;
            }
            let mut position = file.position as usize;
            // if it's space, then fill it from the end
            if file.id == -1 {
                let mut space = file.size;
                while space > 0 {
                    if space >= borrow_file.size {
                        let new_file = File::new(borrow_file.id, total_used as i32, borrow_file.size);
                        c_fs.files.push(new_file);
                        space -= new_file.size;
                        position += new_file.size as usize;
                        total_used += new_file.size;
                        borrow_file = reversed.next().unwrap();
                    } else {
                        let new_file_a = File::new(borrow_file.id, position as i32, space);
                        let new_file_b = File::new(borrow_file.id, borrow_file.position, borrow_file.size - space);
                        c_fs.files.push(new_file_a);
                        space = 0;
                        position += new_file_a.size as usize;
                        total_used += new_file_a.size;
                        borrow_file = new_file_b;
                    }
                }
            // otherwise just add it to the filesystem, no need to split
            } else {
                let remaining_space = self.total_used - total_used;
                if remaining_space < file.size {
                    let new_file = File::new(file.id, file.position, remaining_space);
                    c_fs.files.push(new_file);
                    total_used += new_file.size;
                } else {
                    c_fs.files.push(file);
                    total_used += file.size;
                }
            }
        });
        return c_fs;
    }

    fn move_file(&mut self, file: File, position: i32) {
        let index = self.files.iter().position(|f| f.position == position).unwrap();
        let mut original_position = self.files.iter().position(|f| f.id == file.id).unwrap();
        let mut new_space = File::new(-1, position + file.size, self.files[index].size - file.size);
        self.files[index] = File::new(file.id, position, file.size);
        if new_space.size > 0 {
            self.files.insert(index + 1, new_space);
            original_position += 1;
        }
        self.files[original_position].id = -1;
    }

    fn better_compact(&mut self) -> FileSystem {
        let mut c_fs = FileSystem::new();
        let new_files = self.files.clone();
        c_fs.files = new_files.clone();

        let mut reverse = new_files.clone().into_iter().filter(|f| f.id != -1).collect::<Vec<File>>();
        reverse.reverse();
        let reversed = reverse.into_iter();

        for file in reversed.clone() {
            let space = file.size;
            for check_file in c_fs.files.clone() {
                if check_file.size >= space && check_file.id == -1 && check_file.position < file.position {
                    c_fs.move_file(file, check_file.position);
                    break;
                }
            }
        }

        return c_fs;
    }

    fn string_rep(&self) -> String {
        let mut owned_string: String = "".to_owned();

        for file in self.files.clone() {
            let size = file.size;
            let id = match file.id {
                -1 => ".".to_string(),
                _ => format!("{}", file.id)
            };
            let another_owned_string: String = id.repeat(size as usize).to_owned();
            owned_string.push_str(&another_owned_string);
        }
        return owned_string;
    }
}

fn create_filesystem(data: String) -> FileSystem {
    let mut file_system = FileSystem::new();
    let mut position = 0;
    data.chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            let size = c.to_digit(10).unwrap() as i32;
            file_system.add_file(position, size);
            position += size;
        } else {
            let size = c.to_digit(10).unwrap() as i32;
            if size == 0 {
                return;
            }
            file_system.add_freespace(position, size);
            position += size;
        }
    });
    return file_system;
}

fn analyze_data(data: String) -> (i64, i64) {
    let mut file_system = create_filesystem(data);
    let new_fs = file_system.compact();
    let better_fs = file_system.better_compact();
    let chk_sum = check_sum(new_fs);
    let better_chk_sum = check_sum(better_fs);
    return (chk_sum, better_chk_sum);
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
        assert_eq!(result, 6421128769094);
        assert_eq!(result2, 6448168620520);
    }

    #[test]
    fn test_example_1() {
        let data = String::from(
"2333133121414131402");
        let (result1, result2) = analyze_data(data.clone());
        assert_eq!(result1, 1928);
        assert_eq!(result2, 2858);
    }

    #[test]
    fn test_fs_1() {
        let data = String::from(
"2333133121414131402");
        let mut file_system = create_filesystem(data.clone());
        let string_rep = file_system.string_rep();
        assert_eq!(string_rep, "00...111...2...333.44.5555.6666.777.888899");

        assert_eq!(file_system.files, vec![
            File::new(0, 0, 2),
            File::new(-1, 2, 3),
            File::new(1, 5, 3),
            File::new(-1, 8, 3),
            File::new(2, 11, 1),
            File::new(-1, 12, 3),
            File::new(3, 15, 3),
            File::new(-1, 18, 1),
            File::new(4, 19, 2),
            File::new(-1, 21, 1),
            File::new(5, 22, 4),
            File::new(-1, 26, 1),
            File::new(6, 27, 4),
            File::new(-1, 31, 1),
            File::new(7, 32, 3),
            File::new(-1, 35, 1),
            File::new(8, 36, 4),
            File::new(9, 40, 2)
        ]);

        let new_fs = file_system.compact();
        let string_rep = new_fs.string_rep();
        assert_eq!(string_rep, "0099811188827773336446555566");

        assert_eq!(new_fs.files, vec![
            File::new(0, 0, 2),
            File::new(9, 2, 2),
            File::new(8, 4, 1),
            File::new(1, 5, 3),
            File::new(8, 8, 3),
            File::new(2, 11, 1),
            File::new(7, 12, 3),
            File::new(3, 15, 3),
            File::new(6, 18, 1),
            File::new(4, 19, 2),
            File::new(6, 21, 1),
            File::new(5, 22, 4),
            File::new(6, 26, 1),
            File::new(6, 27, 1)
        ]);
    }

    #[test]
    fn test_fs_2() {
        let data = String::from(
"2333133121414131402");
        let mut file_system = create_filesystem(data.clone());
        let string_rep = file_system.string_rep();
        assert_eq!(string_rep, "00...111...2...333.44.5555.6666.777.888899");

        let new_fs = file_system.better_compact();
        let string_rep = new_fs.string_rep();
        assert_eq!(string_rep, "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn test_fs_move_1() {
        let data = String::from(
            "2333133121414131402");
        let mut file_system = create_filesystem(data.clone());
        file_system.move_file(File::new(9, 40, 2), 2);
        let string_rep = file_system.string_rep();
        assert_eq!(string_rep, "0099.111...2...333.44.5555.6666.777.8888..");
        file_system.move_file(File::new(2, 12, 1), 4);
        let string_rep = file_system.string_rep();
        assert_eq!(string_rep, "00992111.......333.44.5555.6666.777.8888..");
    }

    #[test]
    fn test_check_sum() {
        let mut file_system = FileSystem::new();
        file_system.add_file(0, 1);
        file_system.add_file(1, 10);
        assert_eq!(check_sum(file_system.clone()), 55);

        file_system.add_file(11, 1);
        assert_eq!(check_sum(file_system), 77);
    }

}
