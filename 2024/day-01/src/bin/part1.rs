fn solve(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();

    let mut number_row1: Vec<u32> = vec![];
    let mut number_row2: Vec<u32> = vec![];

    for l in lines.iter() {
        let a = l.split_whitespace().collect::<Vec<_>>();
        number_row1.push(a.iter().next().unwrap().parse::<u32>().unwrap());
        number_row2.push(a.iter().last().unwrap().parse::<u32>().unwrap());
    }

    number_row1.sort();
    number_row2.sort();

    let mut idx: usize = 0;
    let mut result: u32 = 0;

    for _ in number_row1.iter() {
        if number_row1[idx] < number_row2[idx] {
            result += number_row2[idx]-number_row1[idx];
        } else if number_row1[idx] > number_row2[idx] {
            result += number_row1[idx]-number_row2[idx];
        } else {
            result += 0;
        }
        idx += 1;
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

        assert_eq!(11, solve(input))
    }
}
