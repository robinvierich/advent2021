use std::fs;
use std::env;
use std::format;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = format!("{0}", args[1]);
    println!("filename: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let depths = lines.map(|depth_str| depth_str.trim().parse::<i32>().unwrap());


    const WINDOW_LEN : usize = 3;
    let mut window_depths : [i32; WINDOW_LEN] = [0; WINDOW_LEN];

    let mut oldest_depth : i32 = 0;

    let mut depth_sum : i32 = 0;
    let mut prev_depth_sum : i32 = 0;

    let mut depth_increases : i32 = 0;

    for (i, depth) in depths.enumerate()
    {
        if i >= WINDOW_LEN
        {
            oldest_depth = window_depths[i % WINDOW_LEN];
        }
        window_depths[i % WINDOW_LEN] = depth;


        depth_sum = (depth_sum + depth) - oldest_depth;
        println!("sum: {}", depth_sum);


        if i >= WINDOW_LEN
        {
            if depth_sum > prev_depth_sum
            {
                depth_increases += 1;
                println!("Increase!");
            }

            prev_depth_sum = depth_sum;
        }
    }

    println!("{}", depth_increases);
}
