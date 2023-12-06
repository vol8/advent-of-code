fn replace_spelled_with_digit(line: &str) -> String {
    let mut i = 0;
    let mut line = line.to_owned();
    
    loop {
        println!("At index: {i}");
        let chunk = &line[i..];

        if chunk.starts_with("one") {
            line.replace_range(i..i+3, "1");
            i += "one".len();
        } else if chunk.starts_with("two") {
            line.replace_range(i..i+3, "2");
            i += "two".len();
        } else if chunk.starts_with("three") {
            line.replace_range(i..i+5, "3");
            i += "three".len();
        } else if chunk.starts_with("four") {
            line.replace_range(i..i+4, "4");
            i += "four".len();
        } else if chunk.starts_with("five") {
            line.replace_range(i..i+4, "5");
            i += "five".len();
        } else if chunk.starts_with("six") {
            line.replace_range(i..i+3, "6");
            i += "six".len();
        } else if chunk.starts_with("seven") {
            line.replace_range(i..i+5, "7");
            i += "seven".len();
        } else if chunk.starts_with("eight") {
            line.replace_range(i..i+5, "8");
            i += "eight".len();
        } else if chunk.starts_with("nine") {
            line.replace_range(i..i+4, "9");
            i += "nine".len();
        }

        else {
            i += 1;
        }
        if i > line.len()  {break;}
    }
    println!("{line}");
    line
}


fn solve(input: &str) -> u32 {
    let n = input
        .lines()
        .map(|line| {
            //let line = replace_spelled_with_digit(line);
            replace_spelled_with_digit(line)
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for cl in n.iter() {
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
}
