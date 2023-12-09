// CONCEPT: GO BACK TO THIS AGAIN, THE IDEA IS PROBABLY RIGHT!!!!!

use std::collections::HashMap;

trait Solve {
    fn solve(self) -> u128;
}

impl Solve for &str {
    fn solve(self) -> u128 {
        let mut chunks: Vec<&str> = self.split("\n\n").collect();

        // seed numbers get removed from `chunks` vector and `seed_nums` vector is the vector containing every seed number
        let binding = chunks.remove(0).replace("seed: ", "");
        let mut seed_nums: Vec<&str> = binding.split(" ").collect();
        seed_nums.remove(0);

        // turn into function usages
        // ---------------------------------------------------
        // ---------------------------------------------------
        // seed-to-soil
        let mut seed_soil_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("seed-to-soil map:", "");
        let mut seed_to_soil_num_rows: Vec<&str> = binding.split("\n").collect();
        seed_to_soil_num_rows.remove(0);

        for row in seed_to_soil_num_rows {
            
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();
            seed_soil_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                
                src_range_start += 1;
                dest_range_start += 1;
                seed_soil_map.insert(src_range_start.clone(), dest_range_start.clone());
                println!("{:#?}", seed_soil_map.get(&src_range_start.clone()));
            }
        }

        // soil-to-fertilizer
        let mut soil_fertilizer_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("soil-to-fertilizer map:", "");
        let mut soil_to_fertilizer_num_rows: Vec<&str> = binding.split("\n").collect();
        soil_to_fertilizer_num_rows.remove(0);

        for row in soil_to_fertilizer_num_rows {
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();

            soil_fertilizer_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                src_range_start += 1;
                dest_range_start += 1;
                soil_fertilizer_map.insert(src_range_start.clone(), dest_range_start.clone());
            }
        }

        // fertilizer-to-water
        let mut fertilizer_water_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("fertilizer-to-water map:", "");
        let mut fertilizer_to_water_num_rows: Vec<&str> = binding.split("\n").collect();
        fertilizer_to_water_num_rows.remove(0);

        for row in fertilizer_to_water_num_rows {
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();

            fertilizer_water_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                src_range_start += 1;
                dest_range_start += 1;
                fertilizer_water_map.insert(src_range_start.clone(), dest_range_start.clone());
            }
        }

        // water-to-light
        let mut water_light_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("water-to-light map:", "");
        let mut water_to_light_num_rows: Vec<&str> = binding.split("\n").collect();
        water_to_light_num_rows.remove(0);

        for row in water_to_light_num_rows {
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();

            water_light_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                src_range_start += 1;
                dest_range_start += 1;
                water_light_map.insert(src_range_start.clone(), dest_range_start.clone());
            }
        }

        // light-to-temperature
        let mut light_temperature_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("light-to-temperature map:", "");
        let mut light_to_temperature_num_rows: Vec<&str> = binding.split("\n").collect();
        light_to_temperature_num_rows.remove(0);

        for row in light_to_temperature_num_rows {
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();

            light_temperature_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                src_range_start += 1;
                dest_range_start += 1;
                light_temperature_map.insert(src_range_start.clone(), dest_range_start.clone());
            }
        }

        // temperature-to-humidity
        let mut temperature_humidity_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("temperature-to-humidity map:", "");
        let mut temperature_to_humidity_num_rows: Vec<&str> = binding.split("\n").collect();
        temperature_to_humidity_num_rows.remove(0);

        for row in temperature_to_humidity_num_rows {
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();

            temperature_humidity_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                src_range_start += 1;
                dest_range_start += 1;
                temperature_humidity_map.insert(src_range_start.clone(), dest_range_start.clone());
            }
        }

        // humidity-to-location
        let mut humidity_location_map: HashMap<u128, u128> = HashMap::new();
        let binding = chunks.remove(0).replace("humidity-to-location map:", "");
        let mut humidity_to_location_num_rows: Vec<&str> = binding.split("\n").collect();
        humidity_to_location_num_rows.remove(0);

        for row in humidity_to_location_num_rows {
            let nums: Vec<&str> = row.split(" ").collect();
            let mut dest_range_start = nums[0].trim().parse::<u128>().unwrap();
            let mut src_range_start = nums[1].trim().parse::<u128>().unwrap();
            let range = nums[2].trim().parse::<u128>().unwrap();

            humidity_location_map.insert(src_range_start, dest_range_start);
            for _ in 2..=range {
                src_range_start += 1;
                dest_range_start += 1;
                humidity_location_map.insert(src_range_start.clone(), dest_range_start.clone());
            }
        }
        // ---------------------------------------------------
        // ---------------------------------------------------
        let mut location_nums: Vec<u128> = vec![];

        // turn into function usages
        for seed_num in seed_nums {
            let seed_num = seed_num.trim().parse::<u128>().unwrap();
            let soil_test = seed_soil_map.get(&seed_num);
            let soil_num;
            if soil_test.is_none() {
                soil_num = seed_num;
            } else {
                soil_num = *soil_test.unwrap();
            }

            let fertilizer_test = soil_fertilizer_map.get(&soil_num);
            let fertilizer_num;
            if fertilizer_test.is_none() {
                fertilizer_num = soil_num;
            } else {
                fertilizer_num = *fertilizer_test.unwrap();
            }

            let water_test = fertilizer_water_map.get(&fertilizer_num);
            let water_num;
            if water_test.is_none() {
                water_num = fertilizer_num;
            } else {
                water_num = *water_test.unwrap();
            }

            let light_test = water_light_map.get(&water_num);
            let light_num;
            if light_test.is_none() {
                light_num = water_num;
            } else {
                light_num = *light_test.unwrap();
            }

            let temperature_test = light_temperature_map.get(&light_num);
            let temperature_num;
            if temperature_test.is_none() {
                temperature_num = light_num;
            } else {
                temperature_num = *temperature_test.unwrap();
            }

            let humidity_test = temperature_humidity_map.get(&temperature_num);
            let humidity_num;
            if humidity_test.is_none() {
                humidity_num = temperature_num;
            } else {
                humidity_num = *humidity_test.unwrap();
            }

            let location_test = humidity_location_map.get(&humidity_num);
            let location_num;
            if location_test.is_none() {
                location_num = humidity_num;
            } else {
                location_num = *location_test.unwrap();
            }

            location_nums.push(location_num);
        }

        let mut max = 0;
        let mut min = 0; 
        for loc in location_nums.iter() {
            if loc > &max {
                max = *loc;
            }
        }

        for loc in location_nums {
            if loc < max {
                max = loc;
                min = loc;
            }
        }

        //println!("{:#?}", seed_soil_map);
        //println!("{:#?}", seed_nums);
        min
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = input.solve();

    println!("{}", result);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");

        assert_eq!(35, input.solve());
    }
}
