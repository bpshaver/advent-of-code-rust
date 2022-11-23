pub mod input {
    pub fn head(s: &str) -> Vec<&str> {
        s.lines().take(5).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::input;

    #[test]
    fn head_empty() {
        let empty_vec: Vec<&str> = vec![];
        assert_eq!(input::head(""), empty_vec)
    }

    #[test]
    fn head_one() {
        assert_eq!(input::head("foobar"), vec!["foobar"])
    }

    #[test]
    fn head_one_newline() {
        assert_eq!(input::head("\n"), vec![""])
    }
    #[test]
    fn head_two_newlines() {
        assert_eq!(input::head("\n\n"), vec!["", ""])
    }
    #[test]
    fn head_six() {
        assert_eq!(
            input::head("foo\nbar\nbaz\ndead\nbeef\nfoo"),
            vec!["foo", "bar", "baz", "dead", "beef"]
        )
    }
}
