// works for the example but not for the real input

fn replace_spelled_with_digit(line: &str) -> String {
    let mut i = 0;
    let mut line = line.to_owned();

    loop {
        let chunk = &line[i..];
        // why x? I think because of array stuff idk
        if chunk.starts_with("one") {
            line.replace_range(i..i+3, "1");
            i += "one".len();
        } else if chunk.starts_with("two") {
            line.replace_range(i..i+3, "2xx");
            i += "two".len();
        } else if chunk.starts_with("three") {
            line.replace_range(i..i+5, "3xxxx");
            i += "three".len();
        } else if chunk.starts_with("four") {
            line.replace_range(i..i+4, "4xxx");
            i += "four".len();
        } else if chunk.starts_with("five") {
            line.replace_range(i..i+4, "5xxx");
            i += "five".len();
        } else if chunk.starts_with("six") {
            line.replace_range(i..i+3, "6xx");
            i += "six".len();
        } else if chunk.starts_with("seven") {
            line.replace_range(i..i+5, "7xxxx");
            i += "seven".len();
        } else if chunk.starts_with("eight") {
            line.replace_range(i..i+5, "8xxxx");
            i += "eight".len();
        } else if chunk.starts_with("nine") {
            line.replace_range(i..i+4, "9xxx");
            i += "nine".len();
        }

        else {
            i += 1;
        }
        if i >= line.len()  {break;}
        
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
