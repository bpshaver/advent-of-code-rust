#![warn(missing_docs)]

use std::str::FromStr;

/// Return the first 5 lines of the input string as a vector.
///
/// # Examples
///
/// ```
/// use aoc_utils::input;
///
/// let input = "if\non\na\nwinter's\nnight\na\ntraveler";
///
/// assert_eq!(input::head(input), vec!["if","on","a","winter's","night"]);
/// ```
pub fn head(s: &str) -> Vec<&str> {
    s.lines().take(5).collect()
}

/// Return a vector of items of type T from an input string split
/// on newlines.
///
/// T must implement the std::FromStr trait.
///
/// # Examples
///
/// ```
/// use aoc_utils::input;
///
/// let nums: Vec<i32> = input::get_lines_of_type("1\n2\n3");
///
/// assert_eq!(nums, vec![1,2,3]);
/// ```
pub fn get_lines_of_type<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    let mut res: Vec<T> = Vec::new();
    for line in input.lines() {
        match line.parse() {
            Ok(item) => res.push(item),
            Err(_) => continue,
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_empty() {
        let empty_vec: Vec<&str> = vec![];
        assert_eq!(head(""), empty_vec)
    }

    #[test]
    fn head_one() {
        assert_eq!(head("foobar"), vec!["foobar"])
    }

    #[test]
    fn head_one_newline() {
        assert_eq!(head("\n"), vec![""])
    }
    #[test]
    fn head_two_newlines() {
        assert_eq!(head("\n\n"), vec!["", ""])
    }
    #[test]
    fn head_six() {
        assert_eq!(
            head("foo\nbar\nbaz\ndead\nbeef\nfoo"),
            vec!["foo", "bar", "baz", "dead", "beef"]
        )
    }

    #[test]
    fn get_lines_of_type_all_ints() {
        let sample = "1\n2\n3";
        let result: Vec<u8> = get_lines_of_type(sample);
        assert_eq!(result, vec![1, 2, 3])
    }

    #[test]
    fn get_lines_of_type_not_all_ints() {
        let sample = "1\n2\nthree";
        let result: Vec<u32> = get_lines_of_type(sample);
        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn get_lines_of_type_not_all_ints_trailing_newline() {
        let sample = "1\n2\nthree\n";
        let result: Vec<u32> = get_lines_of_type(sample);
        assert_eq!(result, vec![1, 2])
    }
}
