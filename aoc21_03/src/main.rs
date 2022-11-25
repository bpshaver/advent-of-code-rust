use aoc_utils::input;

const INPUT: &str = include_str!("../input.txt");

fn get_counts(bit_strings: Vec<String>) -> Vec<i32> {
    let mut counts: Vec<i32> = Vec::new();
    for bit_string in bit_strings {
        // Iterating through chars of each string made up of '1's and '0's
        // Will accept lines of any length but will panic if any other chars
        // are found.
        for (i, bit) in bit_string.chars().enumerate() {
            if counts.len() == i {
                counts.push(0)
            }
            match bit {
                '0' => counts[i] -= 1,
                '1' => counts[i] += 1,
                _ => panic!(
                    "Input bit string {} contains unallowed character {}!",
                    bit_string, bit
                ),
            }
        }
    }
    counts
}

fn get_count_at_index(bit_strings: &Vec<String>, index: usize) -> Option<i32> {
    let mut count: i32 = 0;
    for bit_string in bit_strings {
        match bit_string.chars().nth(index) {
            Some('0') => count -= 1,
            Some('1') => count += 1,
            Some(c) => panic!(
                "Input bit string {} contains unallowed character {}!",
                bit_string, c
            ),
            None => return None,
        }
    }
    Some(count)
}

fn part_one(input: &str) -> i32 {
    let counts = get_counts(input::get_lines_of_type::<String>(input));
    let gamma: String = counts
        .iter()
        .map(|i| {
            if i > &0 {
                '1'
            } else if i < &0 {
                '0'
            } else {
                panic!("Even number of 1 and 0 characters!")
            }
        })
        .collect();
    let epsilon: String = gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();
    i32::from_str_radix(&gamma[..], 2).expect("Gamma is binary number")
        * i32::from_str_radix(&epsilon[..], 2).expect("Epsilon is a binary number")
}

fn filter_bit_strings_iteratively(bit_strings: &Vec<String>, use_least: bool) -> String {
    let mut res = bit_strings.clone();
    for index in 0..100 {
        if res.len() == 1 {
            break;
        };
        match get_count_at_index(&res, index) {
            None => {
                break;
            }
            Some(count) => res.retain(|s| match s.chars().nth(index) {
                Some('1') => {
                    if !use_least {
                        count >= 0
                    } else {
                        count < 0
                    }
                }
                Some('0') => {
                    if !use_least {
                        count < 0
                    } else {
                        count >= 0
                    }
                }
                _ => panic!(),
            }),
        }
    }
    res.first().unwrap().clone()
}

fn part_two(input: &str) -> i32 {
    let oxygen_generator_rating =
        filter_bit_strings_iteratively(&input::get_lines_of_type(input), false);
    let c02_scrubber_rating =
        filter_bit_strings_iteratively(&input::get_lines_of_type(input), true);
    i32::from_str_radix(&oxygen_generator_rating[..], 2).expect("Is binary number")
        * i32::from_str_radix(&c02_scrubber_rating[..], 2).expect("Is a binary number")
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part_one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part_one: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_count_at_index_basic() {
        let sample = vec![
            String::from("00000"),
            String::from("11111"),
            String::from("01000"),
        ];
        assert_eq!(get_count_at_index(&sample, 0), Some(-1));
        assert_eq!(get_count_at_index(&sample, 1), Some(1));
        assert_eq!(get_count_at_index(&sample, 8), None);
    }

    #[test]
    #[should_panic(expected = "Input bit string 0101a contains unallowed character a!")]
    fn part_one_should_panic_with_unallowed_character() {
        part_one("00100\n11111\n0101a\n00000");
    }

    #[test]
    fn part_one_sample() {
        let sample =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert_eq!(part_one(sample), 198)
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 1458194)
    }

    #[test]
    fn part_two_sample() {
        let sample =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert_eq!(part_two(sample), 230)
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 2829354)
    }
}
