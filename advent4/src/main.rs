use std::env;
use std::format;
use std::fs;

const SIZE_X : usize = 5;
const SIZE_Y : usize = 5;
const NUM_CELLS : usize = SIZE_X * SIZE_Y;


#[allow(dead_code)]
fn get_y(i_cell: usize) -> usize
{
    return i_cell / SIZE_Y;
}

#[allow(dead_code)]
fn get_x(i_cell: usize) -> usize
{
    return i_cell % SIZE_X;
}

fn get_i_cell(x: usize, y: usize) -> usize
{
    return y * SIZE_X + x;
}

fn get_cell_mark_mask(i_cell: usize) -> i32
{
    return 1 << i_cell;
}

const fn get_winning_marks() -> [i32; SIZE_X + SIZE_Y]
{
    let mut winning_marks: [i32; SIZE_X + SIZE_Y] = [0; SIZE_X + SIZE_Y];


    // Can't use for loops in `const fn`s

    let mut y = 0;
    while y < SIZE_Y
    {
        
        let mut x = 0;
        while x < SIZE_X
        {
            // row marks
            winning_marks[y] = winning_marks[y] | (1 << (x + y * SIZE_X));

            // column marks
            winning_marks[y + SIZE_Y] = winning_marks[y + SIZE_Y] | (1 << (y + x * SIZE_X));

            x += 1
        }

        y += 1;
    }

    return winning_marks;
}
    
    
const WINNING_MARKS: [i32; SIZE_X + SIZE_Y] = get_winning_marks();

// const MAX_BINGO_NUM : i32 = 65;


pub enum BingoBoardError
{
    NotFound
}

pub struct BingoBoard
{
    //pub marked : [bool; NUM_CELLS],
    pub marked : i32,
    pub numbers : [i32; NUM_CELLS],

    score : i32,
}


impl BingoBoard
{
    pub fn check_for_win(&self) -> bool
    {
        for winning_mark in WINNING_MARKS 
        {
            if self.marked & winning_mark == winning_mark
            {
                return true;
            }
        }

        return false;
    }

    pub fn mark_cell(&mut self, i_cell: usize)
    {
        let mark_mask = get_cell_mark_mask(i_cell);

        // exit early if already marked
        if (self.marked & mark_mask) > 0
        {
            return;
        }

        self.marked |= mark_mask;

        self.score -= self.numbers[i_cell];
    }

    pub fn get_score(&self) -> i32
    {
        return self.score;
    }

    pub fn get_num(&self, i_cell: usize) -> i32
    {
        return self.numbers[i_cell];
    }

    pub fn get_i_cell_from_num(&self, num:i32) -> Result<usize, BingoBoardError>
    {
        for i_cell in 0..self.numbers.len()
        {
            if self.numbers[i_cell] == num
            {
                return Ok(i_cell);
            }
        }

        return Err(BingoBoardError::NotFound);
    }

    pub fn mark_num(&mut self, num: i32)
    {
        let result = self.get_i_cell_from_num(num);

        match result
        {
            Ok(i_cell) => self.mark_cell(i_cell),
            Err(_) => (),
        }
    }

    pub fn print_board(&self)
    {
        for y in 0..SIZE_Y
        {
            for x in 0..SIZE_X
            {
                let i_cell = get_i_cell(x, y);

                let num = self.get_num(i_cell);

                print!("{:3}", num);
            }
            println!("")
        }
    }

    pub fn new(numbers: [i32; 25]) -> BingoBoard
    {
        BingoBoard { 
            numbers: numbers ,
            marked: 0,
            score: numbers.iter().fold(0, |acc, num| acc + num),
        }  
    }
}


#[allow(dead_code)]
fn part_1(lines_vec: &Vec<&str>)
{
    let mut boards : Vec<BingoBoard> = vec![];

    let drawn_number_strs = lines_vec[0].trim().split(',');
    let drawn_numbers = drawn_number_strs.map(|num_str| num_str.parse::<i32>().unwrap());

    let winning_marks_loc = get_winning_marks();

    for winning_mark in winning_marks_loc {
        println!("{:032b}", winning_mark);
    }


    let mut board_numbers : [i32; NUM_CELLS] = [0; NUM_CELLS];

    let mut y = 0;

    for i_line in 2..lines_vec.len() 
    {
        let line = lines_vec[i_line];

        let is_board_row_line = line.len() > 2;
        let is_last_line = i_line == (lines_vec.len() - 1);

        if is_board_row_line 
        {
            let line_num_strs : Vec<&str> = line.split_whitespace().collect();
            
            for x in 0..line_num_strs.len()
            {
                let num = line_num_strs[x].parse::<i32>().unwrap();

                let i_cell = get_i_cell(x, y);

                board_numbers[i_cell] = num;
            }

            y += 1
        }

        if !is_board_row_line || is_last_line
        {
            let board = BingoBoard::new(board_numbers);

            println!("new board! ");
            board.print_board();
           
            boards.push(board);

            y = 0;
        }
    }

    let mut final_score = 0;

    let mut i_last_board : i32 = -1;

    for drawn_num in drawn_numbers {

        //for board in &mut boards 
        let mut i_board = 0;
        //for i_board in 0..(&mut boards).len()


        while boards.len() > 0 && i_board < boards.len()
        {
            let board = &mut boards[i_board];

            board.mark_num(drawn_num);

            if board.check_for_win()
            {
                if boards.len() == 1
                {
                    i_last_board = i_board as i32;
                    break;
                }
                else
                {
                    boards.remove(i_board);
                }
            }
            else
            {
                i_board += 1;
            }
        }

        if i_last_board >= 0
        {
            let board_score = boards[0].get_score();

            println!("last_board_index {}", i_board);

            println!("board_score {}", board_score);
            println!("last drawn num {}", drawn_num);
    
            final_score = board_score * drawn_num;
            
            break;
        }
        
    }

    println!("final_score {}", final_score);
}





fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = format!("{0}", args[1]);
    println!("filename: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let lines_vec: Vec<&str> = lines.collect();

    part_1(&lines_vec);
}
