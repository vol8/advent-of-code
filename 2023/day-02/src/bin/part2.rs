trait Solve {
    fn get_power(self) -> u32;
}

impl Solve for &str {
    fn get_power(self) -> u32 {
        let mut splitted_game: Vec<&str> = self.split(":").collect();
        let sets: Vec<&str> = splitted_game.remove(1).split(";").collect();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in sets {
            let cubes: Vec<&str> = set.split(",").collect();
            for cube in cubes {
                if cube.contains("red") {
                    let val: u32 = cube
                        .replace("red", "")
                        .trim()
                        .parse::<u32>()
                        .expect("Error: Smth went wrong in calling `parse::<i32>()`");
                    if max_red < val {
                        max_red = val;
                    }
                } else if cube.contains("green") {
                    let val: u32 = cube
                        .replace("green", "")
                        .trim()
                        .parse::<u32>()
                        .expect("Error: Smth went wrong in calling `parse::<i32>()`");
                    if max_green < val {
                        max_green = val;
                    }
                } else if cube.contains("blue") {
                    let val: u32 = cube
                        .replace("blue", "")
                        .trim()
                        .parse::<u32>()
                        .expect("Error: Smth went wrong in calling `parse::<i32>()`");
                    if max_blue < val {
                        max_blue = val;
                    }
                }
            }
        }

        max_red * max_green * max_blue
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../../input1.txt").split("\n").collect();

    let mut result = 0;

    for game in input {
        let power = game.get_power();
        result += power;
    }
    println!("{result}");
}
