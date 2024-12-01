// works for the example but not for the real input

fn replace_spelled_with_digit(line: &str) -> String {
    let mut i = 0;
    let mut line = line.to_owned();

    loop {
        let chunk = &line[i..line.len()];

        // ---------------------------------------------------------
        // Problem: eighttwone => 8ight2w1ne => 821 => 81 and not 82
        // ---------------------------------------------------------
        // And because of `twone` (the o is used by both) we only increment i by 1 so that for `one` in `twone` it uses the o after `two` got replaced => `2wone`!
        if chunk.starts_with("one") {
            line.replace_range(i..i + 3, "1ne");
            i += 1;
        } else if chunk.starts_with("two") {
            line.replace_range(i..i + 3, "2wo");
            i += 1;
        } else if chunk.starts_with("three") {
            line.replace_range(i..i + 5, "3hree");
            i += 1;
        } else if chunk.starts_with("four") {
            line.replace_range(i..i + 4, "4our");
            i += 1;
        } else if chunk.starts_with("five") {
            line.replace_range(i..i + 4, "5ive");
            i += 1;
        } else if chunk.starts_with("six") {
            line.replace_range(i..i + 3, "6ix");
            i += 1;
        } else if chunk.starts_with("seven") {
            line.replace_range(i..i + 5, "7even");
            i += 1;
        } else if chunk.starts_with("eight") {
            line.replace_range(i..i + 5, "8ight");
            i += 1;
        } else if chunk.starts_with("nine") {
            line.replace_range(i..i + 4, "9ine");
            i += 1;
        } else {
            i += 1;
        }

        if i == line.len() {
            break;
        }
    }
    line
}

// From part1.rs
fn solve(input: &str) -> u32 {
    let n: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            replace_spelled_with_digit(line)
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for cl in n {
        let n1 = cl[0].to_digit(10).unwrap();
        let n2 = cl.iter().rev().collect::<Vec<_>>()[0].to_digit(10).unwrap();
        result += n1 * 10 + n2;
    }

    result
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(281, solve(input))
    }

    #[test]
    fn it_works_chris_biscardi() {
        let input = include_str!("../../chris_biscardi_input.txt");
        assert_eq!(54925, solve(input))
    }
}
