use std::env;
use std::format;
use std::fmt;
use std::fs;

struct Point
{
    x : f32,
    y : f32
}


impl fmt::Display for Point
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.x, self.y)
    }
}


enum Orientation
{
    None, // used for points
    Horizontal,
    Vertical,
    Diagonal,
}

impl fmt::Display for Orientation
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let s = match self
        {
            Orientation::None => "None",
            Orientation::Horizontal => "Horizontal",
            Orientation::Vertical => "Vertical",
            Orientation::Diagonal => "Diagonal",
        };

        write!(f, "{}", s)
    }
}


struct Segment
{
    pub pt1 : Point,
    pub pt2 : Point,

    pub ori: Orientation,
}


impl Segment
{
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Segment 
    {
        let ori; 

        if (x1 == x2) && (y1 != y2)
        {
            ori = Orientation::Vertical;
        }
        else if (y1 == y2) && (x1 != x2)
        {
            ori = Orientation::Horizontal;
        }
        else if (x1 == x2) && (y1 == y2)
        {
            // same point
            ori = Orientation::None;
        }
        else
        {
            ori = Orientation::Diagonal;
        }

        Segment
        {
            pt1 : Point {x: x1, y: y1},
            pt2 : Point {x: x2, y: y2},
            ori : ori,
        }
    }

    pub fn x_min(&self) -> f32
    {
        return f32::min(self.pt1.x, self.pt2.x);
    }

    pub fn x_max(&self) -> f32
    {
        return f32::max(self.pt1.x, self.pt2.x);
    }

    pub fn y_min(&self) -> f32
    {
        return f32::min(self.pt1.y, self.pt2.y);
    }

    pub fn y_max(&self) -> f32
    {
        return f32::max(self.pt1.y, self.pt2.y);
    }
}


impl fmt::Display for Segment
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:04} -> {:04}  [{:10}]  ", self.pt1.to_string(), self.pt2.to_string(), self.ori.to_string())
    }
}


pub struct Board
{
    pub x_max : f32,
    pub y_max : f32,

    pub marked : Vec<bool>,
    num_marked: i32,
}

impl Board
{
    pub fn get_i_cell(&self, x: f32, y: f32) -> usize
    {
        let x_int = f32::ceil(x) as usize;

        let stride = self.x_max + 1.;

        let i_row_start = f32::ceil(y * stride) as usize;

        return i_row_start + x_int;
    }

    pub fn new(x_max: f32, y_max : f32) -> Board
    {
        let num_cells = (f32::ceil(x_max + 1.) * f32::ceil(y_max + 1.)) as usize;

        Board
        {
            x_max: x_max,
            y_max: y_max,
            marked: vec![false; num_cells],
            num_marked: 0,
        }
    }

    pub fn mark(&mut self, x: f32, y: f32)
    {
        let i_cell = self.get_i_cell(x, y);

        if !self.marked[i_cell]
        {
            self.marked[i_cell] = true;
            self.num_marked += 1;
        }
    }

    pub fn get_num_marked(&self) -> i32
    {
        return self.num_marked;
    }

    pub fn print(&self)
    {
        let mut y = 0.;
        while y <= self.y_max
        {
            let mut x = 0.;
            while x <= self.x_max
            {
                let i_cell = self.get_i_cell(x, y);
                let is_cell_marked = self.marked[i_cell];

                match is_cell_marked
                {
                    true  => { print!("1"); },
                    false => { print!("."); },
                }

                x += 1.;
            }

            println!("");

            y += 1.;
        }
    }
}




#[allow(dead_code)]
fn part_1(lines_vec: &Vec<&str>)
{

    let mut segments : Vec<Segment> = vec![];

    let mut x_max = 0.;
    let mut y_max = 0.;

    for line in lines_vec
    {

        let split_line : Vec<&str> = line.split("->").collect();

        let coords1 : Vec<f32> = split_line[0].split(",").map(|s| s.trim().parse::<f32>().unwrap()).collect();
        let coords2 : Vec<f32> = split_line[1].split(",").map(|s| s.trim().parse::<f32>().unwrap()).collect();

        let s = Segment::new(coords1[0], coords1[1], coords2[0], coords2[1]);

        x_max = f32::max(f32::max(x_max, s.pt1.x), s.pt2.x);
        y_max = f32::max(f32::max(y_max, s.pt1.y), s.pt2.y);

        segments.push(s);
    }

    // overlap possbilities:
    //  - orthogonal intersection
    //  - parallel overlap

    let mut board : Board = Board::new(x_max, y_max);

    for i_seg in 0..(&segments).len()
    {
        let seg1 = &segments[i_seg];

        for j_seg in (i_seg + 1)..(&segments).len()
        {
            let seg2 = &segments[j_seg];

            match (&seg1.ori, &seg2.ori)
            {
                (Orientation::Horizontal, Orientation::Horizontal) =>
                {
                    if seg1.pt1.y == seg2.pt1.y
                    {
                        let x_start = f32::max(seg1.x_min(), seg2.x_min());
                        let x_end = f32::min(seg1.x_max(), seg2.x_max());

                        println!("h, h: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("x_start, x_end: {}, {}", x_start, x_end);

                        // ys are the same
                        let y = seg1.pt1.y;
                        let mut x = x_start;
                        while x <= x_end
                        {
                            board.mark(x, y);
                            x += 1.;
                        }
                    }
                }

                (Orientation::Vertical, Orientation::Vertical) =>
                {
                    if seg1.pt1.x == seg2.pt1.x
                    {
                        let y_start = f32::max(seg1.y_min(), seg2.y_min());
                        let y_end = f32::min(seg1.y_max(), seg2.y_max());

                        println!("v, v: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("y_start, y_end: {}, {}", y_start, y_end);

                        // xs are the same
                        let x = seg1.pt1.x;
                        let mut y = y_start;
                        while y <= y_end
                        {
                            board.mark(x, y);
                            y += 1.;
                        }
                    }
                }

                (Orientation::Vertical, Orientation::Horizontal) =>
                {
                    let (x, y) = (seg1.pt1.x, seg2.pt1.y);

                    if (seg2.x_min() <= x && x <= seg2.x_max())
                    && (seg1.y_min() <= y && y <= seg1.y_max())
                    {
                        board.mark(x, y);

                        println!("v, h: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("x, y: {}, {}", x, y);

                    }
                }

                (Orientation::Horizontal, Orientation::Vertical) =>
                {
                    let (x, y) = (seg2.pt1.x, seg1.pt1.y);
                    if (seg1.x_min() <= x && x <= seg1.x_max())
                    && (seg2.y_min() <= y && y <= seg2.y_max())
                    {
                        board.mark(x, y);

                        println!("h, v: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("x, y: {}, {}", x, y);
                    }
                }

                // (Orientation::None, Orientation::None) =>
                // {
                //     // if points are the same, mark it
                //     board.mark(x, y);
                // }

                (_, _) =>
                {

                }
            }


            //println!("Board after {}, {}", seg1.to_string(), seg2.to_string());
            //board.print();
        }
    }

    println!("Num Dangerous Areas: {}", board.get_num_marked());
}

#[allow(dead_code)]
fn part_2(lines_vec: &Vec<&str>)
{

    let mut segments : Vec<Segment> = vec![];

    let mut x_max = 0.;
    let mut y_max = 0.;

    for line in lines_vec
    {

        let split_line : Vec<&str> = line.split("->").collect();

        let coords1 : Vec<f32> = split_line[0].split(",").map(|s| s.trim().parse::<f32>().unwrap()).collect();
        let coords2 : Vec<f32> = split_line[1].split(",").map(|s| s.trim().parse::<f32>().unwrap()).collect();

        let s = Segment::new(coords1[0], coords1[1], coords2[0], coords2[1]);

        x_max = f32::max(f32::max(x_max, s.pt1.x), s.pt2.x);
        y_max = f32::max(f32::max(y_max, s.pt1.y), s.pt2.y);

        segments.push(s);
    }

    // overlap possbilities:
    //  - orthogonal intersection
    //  - parallel overlap

    let mut board : Board = Board::new(x_max, y_max);

    for i_seg in 0..(&segments).len()
    {
        let seg1 = &segments[i_seg];

        for j_seg in (i_seg + 1)..(&segments).len()
        {
            let seg2 = &segments[j_seg];

            match (&seg1.ori, &seg2.ori)
            {
                (Orientation::Horizontal, Orientation::Horizontal) =>
                {
                    if seg1.pt1.y == seg2.pt1.y
                    {
                        let x_start = f32::max(seg1.x_min(), seg2.x_min());
                        let x_end = f32::min(seg1.x_max(), seg2.x_max());

                        println!("h, h: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("x_start, x_end: {}, {}", x_start, x_end);

                        // ys are the same
                        let y = seg1.pt1.y;
                        let mut x = x_start;
                        while x <= x_end
                        {
                            board.mark(x, y);
                            x += 1.;
                        }
                    }
                }

                (Orientation::Vertical, Orientation::Vertical) =>
                {
                    if seg1.pt1.x == seg2.pt1.x
                    {
                        let y_start = f32::max(seg1.y_min(), seg2.y_min());
                        let y_end = f32::min(seg1.y_max(), seg2.y_max());

                        println!("v, v: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("y_start, y_end: {}, {}", y_start, y_end);

                        // xs are the same
                        let x = seg1.pt1.x;
                        let mut y = y_start;
                        while y <= y_end
                        {
                            board.mark(x, y);
                            y += 1.;
                        }
                    }
                }

                (Orientation::Vertical, Orientation::Horizontal) =>
                {
                    let (x, y) = (seg1.pt1.x, seg2.pt1.y);

                    if (seg2.x_min() <= x && x <= seg2.x_max())
                    && (seg1.y_min() <= y && y <= seg1.y_max())
                    {
                        board.mark(x, y);

                        println!("v, h: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("x, y: {}, {}", x, y);

                    }
                }

                (Orientation::Horizontal, Orientation::Vertical) =>
                {
                    let (x, y) = (seg2.pt1.x, seg1.pt1.y);
                    if (seg1.x_min() <= x && x <= seg1.x_max())
                    && (seg2.y_min() <= y && y <= seg2.y_max())
                    {
                        board.mark(x, y);

                        println!("h, v: {}, {}", seg1.to_string(), seg2.to_string());
                        println!("x, y: {}, {}", x, y);
                    }
                }

                // (Orientation::None, Orientation::None) =>
                // {
                //     // if points are the same, mark it
                //     board.mark(x, y);
                // }

                (_, _) =>
                {

                }
            }


            //println!("Board after {}, {}", seg1.to_string(), seg2.to_string());
            //board.print();
        }
    }

    println!("Num Dangerous Areas: {}", board.get_num_marked());
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
