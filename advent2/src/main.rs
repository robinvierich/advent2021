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

    let mut pos : [i32; 2] = [0, 0];

    let mut aim : i32 = 0;
    let mut aim_dir : i32 = 0;

    for line in lines
    {
        let split_line : Vec<&str> = line.split(" ").collect();

        let dir_str = split_line[0].trim();
        let mag_str = split_line[1].trim();

        let mut dir : [i32; 2] = [0, 0];

        match dir_str
        {
            "forward" => {dir = [1, 0]; aim_dir = 0;}
            "down" => {dir = [0, 0]; aim_dir = 1;}
            "up" => {dir = [0, 0]; aim_dir = -1;}
            &_ => {}
        }

        let mag = mag_str.parse::<i32>().unwrap();

        aim += aim_dir * mag;

        if dir[0] != 0
        {
            pos[0] += mag * dir[0];
            pos[1] += aim * mag;
        }

        println!("pos: {}, {}   aim: {}", pos[0], pos[1], aim);
    }
}
