use std::fs;
use std::env;
use std::format;


fn part_1<'a>(lines : &Vec<&str>)
{
    let first_line = lines[0];

    let readings = lines.iter().map(|line| i32::from_str_radix(line.trim(), 2).unwrap());

    let num_readings = readings.len();
    let half_num_readings = (num_readings / 2) as i32;

    let binary_width = first_line.trim().len();

    let mut ones : Vec<i32> = vec![0; binary_width];

    for reading in readings
    {
        for i in 0..binary_width
        {
            ones[i] += (reading >> i) & 1;
        }
    }

    let mut gamma_rate_str : String = String::with_capacity(binary_width);

    for i in 0..binary_width
    {
        let idx = binary_width - i - 1;

        let num_ones_for_bit = ones[idx];

        if num_ones_for_bit >= half_num_readings
        {
            gamma_rate_str.push('1');
        }
        else
        {
            gamma_rate_str.push('0');
        }

    }

    let width_mask = (1 << binary_width) - 1;

    let gamma_rate = i32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = (!gamma_rate) & width_mask;


    println!("gamma   = {:8} = {:012b}", gamma_rate, gamma_rate);
    println!("epsilon = {:8} = {:012b}", epsilon_rate, epsilon_rate);

    let power = gamma_rate * epsilon_rate;

    println!("power   = {:8}", power);
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = format!("{0}", args[1]);
    println!("filename: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    part_1(&lines.collect());
}

