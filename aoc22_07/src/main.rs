use aoc_utils::input;
mod fs;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> fs::FileSystem {
    let mut lines = input.trim().lines();
    if lines.next().expect("Input should have at least one line") != "$ cd /" {
        panic!("First line should be '$ cd /'")
    }
    let mut fs = fs::FileSystem::new();
    for line in lines {
        if line == "$ cd .." {
            fs.cd("..").expect("'cd ..' always succeeds");
        } else if line.starts_with("$ cd") {
            let dirname = line
                .split(' ')
                .nth(2)
                .expect("Line starting '$ cd' has pattern '$ cd dir'");
            match fs.cd(dirname) {
                Ok(_) => continue,
                Err(_) => {
                    fs.mkdir(dirname);
                    fs.cd(dirname)
                        .expect("dirname is valid because we just created it")
                }
            }
        } else if line.starts_with("$ ls") {
            // We don't do anything in this case, we just wait and see what comes up next
            continue;
        } else if line.starts_with("dir") {
            let (_, name) = line
                .split_once(' ')
                .expect("Lines starting with 'dir' have pattern 'dir dirname'");
            if !fs.exists(name) {
                fs.mkdir(name)
            }
        } else {
            let (size, name) = line
                .split_once(' ')
                .expect("Every other line should have pattern 'size filename.suffix'");

            if !fs.exists(name) {
                fs.touch(name, size.parse().expect("Size should be integer"));
            }
        }
    }
    fs.cd_root();
    fs
}

fn part_one(input: &str) -> usize {
    let fs = parse_input(input);
    let dir_sizes = fs.rget_all_dir_sizes();
    dir_sizes.iter().filter(|size| size < &&100000).sum()
}

fn part_two(input: &str) -> usize {
    let fs = parse_input(input);
    let dir_sizes = fs.rget_all_dir_sizes();
    let required_space = 30000000 - (70000000 - fs.duh());
    *dir_sizes
        .iter()
        .filter(|size| size > &&required_space)
        .min()
        .expect("There should be at least one directory larger than 'required_space'")
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    #[test]
    fn parse_input_sample() {
        let fs = parse_input(SAMPLE);
        assert_eq!(fs.duh(), 48381165);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 95437);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 1367870);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 24933642);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 549173);
    }
}
