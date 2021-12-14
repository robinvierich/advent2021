use std::env;
use std::fs;
use std::path::Path;


fn calc_cost(final_x : i32, num_crabs_by_x: &Vec<i32>) -> i32
{
    let mut cost : i32 = 0; 

    for x in 0..num_crabs_by_x.len()
    {
        let dist : i32 = i32::abs(final_x - (x as i32));
        //let dist_sqr = i32::pow(dist, 2);

        let num_crabs = num_crabs_by_x[x];

        cost += num_crabs * dist
    }

    return cost;
}


fn calc_dist_cost_2(dist : i32) -> i32
{
    return dist * (dist + 1) / 2;
}


fn calc_cost_2(final_x : i32, num_crabs_by_x: &Vec<i32>) -> i32
{
    let mut cost : i32 = 0; 

    for x in 0..num_crabs_by_x.len()
    {
        let dist : i32 = i32::abs(final_x - (x as i32));

        let num_crabs = num_crabs_by_x[x];

        let dist_cost = calc_dist_cost_2(dist);

        cost += num_crabs * dist_cost;
    }

    return cost;
}


fn part_1(lines_vec: &Vec<&str>) {

    let split_line = lines_vec[0].trim().split(',');

    let xs : Vec<i32> = split_line.map(|x| x.parse::<i32>().unwrap()).collect();

    let mut min_x = i32::MAX;
    let mut max_x : i32 = -1;

    let mut num_crabs_by_x : Vec<i32> = vec![];

    for x in xs
    {
        if x < min_x
        {
            min_x = x;
        }

        if x > max_x
        {
            max_x = x;
        }

        let i_x = x as usize;

        if num_crabs_by_x.len() <= i_x
        {
            num_crabs_by_x.resize(i_x + 1, 0);
        }

        num_crabs_by_x[i_x] += 1;
    }

    let mut min_cost = i32::MAX;
    let mut x_min_cost = -1;

    // Brute force.. Too Bad!
    for x in min_x..=max_x
    {
        let cost = calc_cost(x, &num_crabs_by_x);

        println!("{:2} cost: {}", x, cost);

        if cost < min_cost
        {
            min_cost = cost;
            x_min_cost = x;
        }
    }

    println!("min_cost is {}  @  x = {}", min_cost, x_min_cost);
}

#[allow(dead_code)]
fn part_2(lines_vec: &Vec<&str>) {
    let split_line = lines_vec[0].trim().split(',');

    let xs : Vec<i32> = split_line.map(|x| x.parse::<i32>().unwrap()).collect();

    let mut min_x = i32::MAX;
    let mut max_x : i32 = -1;

    let mut num_crabs_by_x : Vec<i32> = vec![];

    for x in xs
    {
        if x < min_x
        {
            min_x = x;
        }

        if x > max_x
        {
            max_x = x;
        }

        let i_x = x as usize;

        if num_crabs_by_x.len() <= i_x
        {
            num_crabs_by_x.resize(i_x + 1, 0);
        }

        num_crabs_by_x[i_x] += 1;
    }

    let mut min_cost = i32::MAX;
    let mut x_min_cost = -1;

    // Brute force.. Too Bad!
    for x in min_x..=max_x
    {
        let cost = calc_cost_2(x, &num_crabs_by_x);

        println!("{:2} cost: {}", x, cost);

        if cost < min_cost
        {
            min_cost = cost;
            x_min_cost = x;
        }
    }

    println!("min_cost is {}  @  x = {}", min_cost, x_min_cost);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //let path = Path::new(args[1].as_str()).join("sample_input.txt");
    let path = Path::new(args[1].as_str()).join("input.txt");

    println!("input_file: {}", path.to_str().unwrap());

    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let lines_vec: Vec<&str> = lines.collect();

    //part_1(&lines_vec);
    part_2(&lines_vec);
}
