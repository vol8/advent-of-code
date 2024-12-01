trait Solve {
    fn solve(self) -> u32;
}

const BASE: u32 = 2;

impl Solve for &str {
    fn solve(self) -> u32 {
        let mut w_count: u32 = 0;
        let mut result: u32 = 0;
        let binding = self.replace("  ", " 0");
        let cards: Vec<&str> = binding.split("\n").collect();

        for card in cards {
            let mut splitted_card: Vec<&str> = card.split(": ").collect();
            let number_list: Vec<&str> = splitted_card.remove(1).split(" | ").collect();

            let winning_numbers: &Vec<&str> = &number_list[0].split(' ').collect();
            let num_elf_got: &Vec<&str> = &number_list[1].split(' ').collect();

            for num in num_elf_got {
                for wnum in winning_numbers {
                    if num == wnum {
                        w_count += 1;
                        break;
                    }
                }
            }
            if w_count > 0 {
                result += BASE.pow(w_count-1);
            }
            w_count = 0;
        }
        result
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", input.solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(13, input.solve())
    }
}