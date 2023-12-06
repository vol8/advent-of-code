fn solve(input: &str) -> u32 {
    let n = input
        .lines()
        .map(|line| {
            line.chars()
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

        assert_eq!(142, solve(input))
    }
}
