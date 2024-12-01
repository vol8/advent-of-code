trait Solve {
    fn solve(self) -> u32;
}

struct Position {
    x: u32,
    y: u32
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self {
            y: y,
            x: x
        }
    }
}

const LENGTH: u32 = 10;

fn get_index_at_pos(x: u32, y: u32) -> usize {
    (x+(y*LENGTH)) as usize - 1
}

impl Solve for &str {
    fn solve(self) -> u32 {
        // To get a character at position (x,y) you have to do it like this: char_list[x + (y*length)]
        let char_list: Vec<char> = self.replace("\n", "").chars()/*.filter(|c| *c != '\n')*/.collect();

        for (n, char) in char_list.iter().enumerate() {
            if !char.is_ascii_digit() && *char != '.'{
                //let symbol_pos: Position = Position::new(x, y)
            } 
        }

        let t = char_list[get_index_at_pos(3, 0)];
        println!("{t}");
        todo!()
    }
}

fn main() {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(4361, input.solve());
    }
}