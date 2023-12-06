fn main() {
    let mut final_result = 0;
    let mut temp = String::from("");
    let input = include_str!("../../input.txt");
    let input: Vec<&str> = input.split('\n').collect();

    // removes any other characters except the digits
    for line in input {
        for chars in line.chars() {
            if chars.is_ascii_digit() {
                temp = temp + &chars.to_string();
            }
        }
        temp = temp + "\n";
    }

    let t = temp.clone();
    let numlist: Vec<&str> = t.split('\n').collect();

    // transforms the digits which length are < and > 2 to a 2 digit number and combines all the numbers together
    for nums in numlist {
        if nums.len() == 1 {
            temp = format!("{}{}", nums, nums);
            final_result += temp.parse::<i32>().unwrap();
        } else if nums.len() > 2 {
            let mut splitnums: Vec<&str> = nums.split("").collect();

            splitnums.remove(0); // removes first ` "" `
            splitnums.remove(splitnums.len() - 1); // removes last ` "" `

            temp = format!(
                "{}{}",
                splitnums.remove(0),
                splitnums.remove(splitnums.len() - 1)
            );

            final_result += temp.parse::<i32>().unwrap();
        } else if nums.len() == 2 {
            final_result += nums.parse::<i32>().unwrap();
        }
    }
    println!("{}", final_result);
}
