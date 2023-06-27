const INPUT: &str = "yzbqklnj";

fn get_md5(s: &str) -> String {
    let digest = md5::compute(s);
    format!("{:x}", digest)
}

fn startswith_five_zeros(s: &str) -> bool {
    s.starts_with("00000")
}

fn startswith_six_zeros(s: &str) -> bool {
    s.starts_with("000000")
}

fn concat_key(key: &str, i: usize) -> String {
    format!("{key}{i}")
}

fn part_one(input: &str) -> usize {
    let mut i = 0;
    while !startswith_five_zeros(&get_md5(&concat_key(input, i))) {
        i += 1
    }
    i
}

fn part_two(input: &str) -> usize {
    // TODO: Make this faster.
    let mut i = 0;
    while !startswith_six_zeros(&get_md5(&concat_key(input, i))) {
        i += 1;
    }
    i
}

fn main() {
    let part_one_solution = part_one(INPUT);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_md5_basic() {
        assert_eq!(get_md5("foobar"), "3858f62230ac3c915f300c664312c63f");
    }

    #[test]
    fn startswith_five_zeros_basic() {
        assert!(startswith_five_zeros("00000foobar"));
        assert!(startswith_five_zeros("00000"));
        assert!(!startswith_five_zeros("0000foobar"))
    }

    #[test]
    fn concat_key_basic() {
        assert_eq!(concat_key("foobar", 1234), "foobar1234");
    }

    #[test]
    fn part_one_actual() {
        assert_eq!(part_one(INPUT), 282749);
    }
}
