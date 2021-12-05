use std::env;
use std::format;
use std::fs;

fn calc_most_common_bits(numbers: &Vec<i32>, bitwidth: usize) -> i32 {
    let mut ones: Vec<i32> = vec![0; bitwidth];

    for num in numbers {
        for i in 0..bitwidth {
            ones[i] += (num >> i) & 1;
        }
    }

    let mut most_common_bits: i32 = 0;

    // add %2 to "ceil" the integer division
    let half_numbers_len = (numbers.len() / 2) + numbers.len() % 2;

    for i in 0..bitwidth {
        let num_ones_for_bit = ones[i];

        if (num_ones_for_bit as usize) >= half_numbers_len {
            most_common_bits = most_common_bits | (1 << i);
        }
    }

    return most_common_bits;
}

fn calc_least_common_bits(numbers: &Vec<i32>, bitwidth: usize) -> i32 {
    let most_common_bits = calc_most_common_bits(&numbers, bitwidth);

    let width_mask = (1 << bitwidth) - 1;
    return (!most_common_bits) & width_mask;
}

fn calc_gamma_and_epsilon_ratings(lines: &Vec<&str>) -> (i32, i32) {
    let first_line = lines[0];

    let readings = lines
        .iter()
        .map(|line| i32::from_str_radix(line.trim(), 2).unwrap());

    let binary_width = first_line.trim().len();

    let gamma_rate = calc_most_common_bits(&readings.collect(), binary_width);

    let width_mask = (1 << binary_width) - 1;
    let epsilon_rate = (!gamma_rate) & width_mask;

    return (gamma_rate, epsilon_rate);
}

#[allow(dead_code)]
fn part_1<'a>(lines: &Vec<&str>) {
    let (gamma_rate, epsilon_rate) = calc_gamma_and_epsilon_ratings(lines);

    println!("gamma   = {:8} = {:012b}", gamma_rate, gamma_rate);
    println!("epsilon = {:8} = {:012b}", epsilon_rate, epsilon_rate);

    let power = gamma_rate * epsilon_rate;

    println!("power   = {:8}", power);
}

#[allow(dead_code)]
fn part_2<'a>(lines: &Vec<&str>) {
    let first_line = lines[0];
    let binary_width = first_line.trim().len();

    let readings: Vec<i32> = lines
        .iter()
        .map(|line| i32::from_str_radix(line.trim(), 2).unwrap())
        .collect();

    // :'(
    let mut most_common_bit_readings = readings.to_vec();
    let mut least_common_bit_readings = readings.to_vec();

    for i in 0..binary_width {
        let bitmask = 1 << (binary_width - i - 1);

        let most_common_bits = calc_most_common_bits(&most_common_bit_readings, binary_width);
        let least_common_bits = calc_least_common_bits(&least_common_bit_readings, binary_width);

        // :'(
        if most_common_bit_readings.len() > 1 {
            let mut i_reading = 0;
            while i_reading < most_common_bit_readings.len() {
                let reading = most_common_bit_readings[i_reading];

                let bit_is_most_common = (most_common_bits & bitmask) == (reading & bitmask);

                if !bit_is_most_common {
                    // swap_remove will not preserve order, but is O(1), so we don't need to worry about iteration direction
                    most_common_bit_readings.swap_remove(i_reading);
                } else {
                    i_reading += 1;
                }

                if most_common_bit_readings.len() <= 1 {
                    break;
                }
            }
        }

        // :'(
        if least_common_bit_readings.len() > 1 {
            let mut i_reading = 0;
            while i_reading < least_common_bit_readings.len() {
                let reading = least_common_bit_readings[i_reading];

                let bit_is_least_common = (least_common_bits & bitmask) == (reading & bitmask);

                if !bit_is_least_common {
                    least_common_bit_readings.swap_remove(i_reading);
                } else {
                    i_reading += 1;
                }

                if least_common_bit_readings.len() <= 1 {
                    break;
                }
            }
        }

        if most_common_bit_readings.len() <= 1 && least_common_bit_readings.len() <= 1 {
            break;
        }
    }

    if most_common_bit_readings.len() != 1 {
        println!("Error! Could not find oxygen_generator_rating! Ran out of options (algorithm must have a bug).");
        return;
    }

    if least_common_bit_readings.len() != 1 {
        println!("Error! Could not find co2_scrubber_rating! Ran out of options (algorithm must have a bug).");
        return;
    }

    let oxygen_generator_rating = most_common_bit_readings[0];
    println!("oxygen_generator_rating = {}", oxygen_generator_rating);

    let co2_scrubber_rating = least_common_bit_readings[0];
    println!("co2_scrubber_rating = {}", co2_scrubber_rating);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
    println!("life_support_rating = {}", life_support_rating);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = format!("{0}", args[1]);
    println!("filename: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let lines_vec: Vec<&str> = lines.collect();

    //part_1(&lines_vec);
    part_2(&lines_vec);
}
