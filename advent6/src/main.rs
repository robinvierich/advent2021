use std::env;
use std::format;
use std::fmt;
use std::path::Path;
use std::fs;

const DAYS_BETWEEN_SPAWNS: usize = 7;
const DAYS_UNTIL_FIRST_SPAWN: usize = DAYS_BETWEEN_SPAWNS + 2;

const NUM_SIM_DAYS : i32 = 256;

struct Fish {
    pub days_until_spawn: i32,
}

impl fmt::Display for Fish
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.days_until_spawn)
    }
}

fn part_1(lines_vec: &Vec<&str>) {
    let initial_spawn_days_csv = lines_vec[0].split(',');

    let initial_days_until_spawn =
        initial_spawn_days_csv.map(|days_str| days_str.parse::<i32>().unwrap());

    let mut fishes: Vec<Fish> = initial_days_until_spawn
        .map(|days| Fish {
            days_until_spawn: days,
        })
        .collect();
    

    print!("initial state:"); 
    for fish in &fishes {
        print!("{},", fish.to_string());
    }

    for _day in 1..=NUM_SIM_DAYS
    {
        //println!("");
        //print!("After {:3} {:4}: ", day, match day { 1 => "day", _ => "days"});


        let mut i_fish = 0;

        while i_fish < (&mut fishes).len()
        {
            //let mut fish = &(&mut fishes)[i_fish];
        
            if fishes[i_fish].days_until_spawn == 0
            {
                //fishes.push(Fish{ days_until_spawn: DAYS_UNTIL_FIRST_SPAWN });
                fishes[i_fish].days_until_spawn = DAYS_BETWEEN_SPAWNS as i32;

                fishes.push(Fish{ days_until_spawn: DAYS_UNTIL_FIRST_SPAWN as i32 });
            }
            
            fishes[i_fish].days_until_spawn -= 1;

            //print!("{},", fishes[i_fish].to_string());

            i_fish += 1;
        }
    }

    println!("");
    println!("num fish: {}", fishes.len());
}

#[allow(dead_code)]
fn part_2(lines_vec: &Vec<&str>) 
{
    let initial_spawn_days_csv = lines_vec[0].split(',');

    let initial_days_until_spawn =
        initial_spawn_days_csv.map(|days_str| days_str.parse::<i32>().unwrap());

    let mut fishes: Vec<Fish> = initial_days_until_spawn
        .map(|days| Fish {
            days_until_spawn: days,
        })
        .collect();
    

    print!("initial state:"); 
    for fish in &fishes {
        print!("{},", fish.to_string());
    }

    let mut num_fishes_with_n_days_remaining : [u64; DAYS_UNTIL_FIRST_SPAWN] = [0; DAYS_UNTIL_FIRST_SPAWN];

    for fish in &fishes 
    {
        num_fishes_with_n_days_remaining[fish.days_until_spawn as usize] += 1;
    }


    for _day in 1..=NUM_SIM_DAYS
    {
        


        let num_new_fishes = num_fishes_with_n_days_remaining[0];
        let num_parent_fishes = num_new_fishes;

        //println!("");
        //println!("new: {}", num_new_fishes);


        for i_days_remaining in 1..DAYS_UNTIL_FIRST_SPAWN 
        {
            num_fishes_with_n_days_remaining[i_days_remaining - 1] = num_fishes_with_n_days_remaining[i_days_remaining];
        }
        num_fishes_with_n_days_remaining[DAYS_UNTIL_FIRST_SPAWN - 1] = num_new_fishes;
        num_fishes_with_n_days_remaining[DAYS_BETWEEN_SPAWNS - 1] += num_parent_fishes;


        println!("");
        print!("After {:2} days: ", _day);
        for i in 0..DAYS_UNTIL_FIRST_SPAWN 
        {
            print!("{},", num_fishes_with_n_days_remaining[i]);
        }
    }

    let mut total_num_fishes = 0;

    for i in 0..DAYS_UNTIL_FIRST_SPAWN 
    {
        total_num_fishes += num_fishes_with_n_days_remaining[i];
    }

    println!("num fish: {}", total_num_fishes);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(args[1].as_str()).join("input.txt");
    //let path = Path::new(args[1].as_str()).join("input.txt");

    println!("input_file: {}", path.to_str().unwrap());

    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let lines_vec: Vec<&str> = lines.collect();

    //part_1(&lines_vec);
    part_2(&lines_vec);
}
