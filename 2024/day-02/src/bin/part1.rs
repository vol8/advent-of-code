fn check_if_dec_inc(nums_in_row: &Vec<u32>) -> bool {
    let mut is_inc = false;
    let mut idx = 0;


    if nums_in_row[0] < nums_in_row[1] {
        is_inc = true;
    } else if nums_in_row[0] == nums_in_row[1] {
        return false;
    }

    for n in nums_in_row.iter() {
        if nums_in_row.len() == idx+1 {
            break;
        }
        //println!("comparing: '{}' with '{}'", n, nums_in_row[idx+1]);
        if is_inc && nums_in_row[idx] > nums_in_row[idx+1] {
            return false;
        } else if !is_inc && nums_in_row[idx] < nums_in_row[idx+1] {
            return false;
        } else if nums_in_row[idx] == nums_in_row[idx+1] {
            return false;
        } 

        idx += 1;
    }
    true
}

fn check_inc_dec_val(nums_in_row: &Vec<u32>) -> bool {
    let mut idx = 0;

    for n in nums_in_row.iter() {
        if nums_in_row.len() == idx+1 {
            break;
        }
        if nums_in_row[idx] > nums_in_row[idx+1] && nums_in_row[idx] - nums_in_row[idx+1] >= 4 {
            return false;
        } else if nums_in_row[idx] < nums_in_row[idx+1] && nums_in_row[idx+1] - nums_in_row[idx] >= 4 {
            return false;
        } 

        idx += 1;
    } 
    true
}

fn solve(input: &str) -> u32 {
    let mut result = 0;
    let mut nums_in_row = vec![];
    let lines = input.lines().collect::<Vec<_>>();

    for l in lines.iter() {
        for c in l.split_whitespace().collect::<Vec<_>>() {
            nums_in_row.push(c.parse::<u32>().unwrap());
        }
        
        if check_if_dec_inc(&nums_in_row) && check_inc_dec_val(&nums_in_row) {
            result += 1;       
        }

        nums_in_row.clear();
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

        assert_eq!(2, solve(input))
    }
}
