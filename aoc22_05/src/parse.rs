fn parse_stack_line(line: &str) -> Vec<char> {
    line.chars()
        .enumerate()
        .filter(|(i, _)| i > &0)
        .filter(|(i, _)| (i - 1) % 4 == 0)
        .map(|(_, c)| if c.is_digit(10) { ' ' } else { c })
        .collect()
}

pub fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = stacks.lines().map(parse_stack_line).collect();
    let mut columns: Vec<Vec<char>> = Vec::new();
    while rows.len() > 0 {
        let row = rows.pop().expect("Rows len is positive");
        if columns.len() == 0 {
            for _ in 0..row.len() {
                columns.push(Vec::new());
            }
        }
        for (i, c) in row.iter().enumerate() {
            if c != &' ' {
                columns.get_mut(i).expect("Column index exists").push(*c);
            }
        }
    }

    columns
}

pub fn parse_instructions(instructions: &str) -> Vec<(usize, usize, usize)> {
    instructions
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let n: usize = split
                .nth(1)
                .expect("Instruction 'n' should not be empty")
                .parse()
                .expect("Instruction 'n' should be an integer");
            let from: usize = split
                .nth(1)
                .expect("Instruction 'from' should not be empty")
                .parse()
                .expect("Instruction 'from' should be an integer");
            let to: usize = split
                .nth(1)
                .expect("Instruction 'to' should not be empty")
                .parse()
                .expect("Instruction 'to' should be an integer");

            (n, from, to)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stack_line_basic() {
        assert_eq!(
            parse_stack_line("        [H]     [W] [B]            "),
            vec![' ', ' ', 'H', ' ', 'W', 'B', ' ', ' ', ' '],
        );
        assert_eq!(
            parse_stack_line("[T] [L] [Z] [R] [C] [Q] [V] [P] [H]"),
            vec!['T', 'L', 'Z', 'R', 'C', 'Q', 'V', 'P', 'H'],
        );
    }

    #[test]
    fn parse_stacks_basic() {
        assert_eq!(
            parse_stacks("    [D]    \n[N] [C]    \n[Z] [M] [P]\n"),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        )
    }
}
