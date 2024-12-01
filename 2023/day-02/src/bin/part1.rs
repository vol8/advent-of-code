use regex::Regex;

trait Solve {
    fn is_possible(self) -> bool;
}

impl Solve for &str {
    fn is_possible(self) -> bool {
        let mut splitted_game: Vec<&str> = self.split(":").collect();

        let rounds: Vec<&str> = splitted_game.remove(1).split(";").collect();

        for round in rounds {
            let cubes: Vec<&str> = round.split(",").collect();
            for cube in cubes {
                if cube.contains("red") {
                    let amount = cube
                        .replace("red", "")
                        .trim()
                        .parse::<i32>()
                        .expect("Error: Smth went wrong in calling `parse::<i32>()`");
                    if amount > 12 {
                        return false;
                    }
                } else if cube.contains("green") {
                    let amount = cube
                        .replace("green", "")
                        .trim()
                        .parse::<i32>()
                        .expect("Error: Smth went wrong in calling `parse::<i32>()`");
                    if amount > 13 {
                        return false;
                    }
                } else if cube.contains("blue") {
                    let amount = cube
                        .replace("blue", "")
                        .trim()
                        .parse::<i32>()
                        .expect("Error: Smth went wrong in calling `parse::<i32>()`");
                    if amount > 14 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../../input1.txt").split("\n").collect();

    let mut result = 0;

    let re = Regex::new(r"Game\s*(\d*)").unwrap();

    for game in input {
        if game.is_possible() {
            let game_title = re.captures(game).unwrap().get(1).unwrap().as_str();
            result += game_title
                .replace("Game ", "")
                .trim()
                .parse::<i32>()
                .unwrap();
        }
    }
    println!("{result}");
}
