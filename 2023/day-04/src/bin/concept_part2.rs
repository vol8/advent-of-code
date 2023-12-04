// This is a concept but I think I almost got.
// Note: Get back to this because it could be done at any time. Just fucking debug the thing. 
//
// |---------------------------------------|
// |Test fails because: assert_eq!(30, 24);|
// |---------------------------------------|
//

use std::collections::HashMap;

trait Parser {
    fn parse_list(self) -> HashMap<u16, String>;
}

trait Solver {
    fn solve(self) -> u32;
}

impl Parser for &str {
    fn parse_list(self) -> HashMap<u16, String> {
        let binding = self.replace("  ", " 0").replace("Card ", "");
        let card_list: Vec<&str> = binding.as_str().split("\n").collect();
        
        let mut card_map: HashMap<u16, String> = HashMap::new();

        for card in card_list {
            let key = card.split(": ").nth(0).unwrap().parse::<u16>().unwrap();
            let val = card.split(": ").nth(1).unwrap().to_string();
            card_map.insert(key, val);
        }
        println!("{:#?}", card_map);
        card_map
    }
}

// Use a hashmap to find the total wins of one card. Afterwards, create a new hashmaps with the cards you copied because of the wins and recursivly solve that.
// Add every win to the result variable and at the end, assert_eq it with 30 because of the example.
// Something is wrong in this code. Fix in some time
impl<'a> Solver for HashMap<u16, String> {
    fn solve(self) -> u32 {
        let mut result = self.len() as u32;
        for t in self.iter() {
            let mut wins: u16 = 0;
            
            let card_num = t.0;
            let number_list: Vec<&str> = t.1.as_str().split(" | ").collect();

            let n: Vec<&str> = number_list[1].split(" ").collect();
            let wn: Vec<&str> = number_list[0].split(" ").collect();

            for n in n.iter() {
                for wn in wn.iter() {
                    if n == wn {
                        wins += 1;
                        break;
                    }
                }
            }
            
            let mut map: HashMap<u16, String> = HashMap::new();

            for i in 1..=wins {
                println!("{i}");
                if self.get(&(card_num + &i)).is_none() {continue;}
                map.insert(*card_num + &i, self.get(&(card_num + &i)).expect("val at key doesnt exist ig").to_owned());
            }            

            result += map.solve();
        }
        result
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", input.parse_list().solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(30, input.parse_list().solve())
    }
}