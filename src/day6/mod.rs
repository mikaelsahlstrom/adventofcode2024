use crate::utils;

enum Direction
{
    Up,
    Down,
    Left,
    Right
}

struct Guard
{
    row: i32,
    pos: i32,
    direction: Direction
}

impl Guard
{
    fn is_outside(&self, board: &Vec<Vec<char>>) -> bool
    {
        return match self.direction
        {
            Direction::Up => self.row < 0,
            Direction::Down => self.row >= (board.len() as i32),
            Direction::Left => self.pos < 0,
            Direction::Right => self.pos >= (board[self.row as usize].len() as i32)
        }
    }

    fn move_position(&mut self, board: &mut Vec<Vec<char>>)
    {
        match self.direction
        {
            Direction::Up =>
            {
                if self.row - 1 >= 0
                {

                    if board[self.row as usize - 1][self.pos as usize] == '#'
                    {
                        self.direction = Direction::Right;
                        self.pos += 1;
                    }
                    else
                    {
                        self.row -= 1;
                    }
                }
                else
                {
                    self.row -= 1;
                }
            },
            Direction::Down =>
            {
                if self.row + 1 < (board.len() as i32)
                {
                    if board[self.row as usize + 1][self.pos as usize] == '#'
                    {
                        self.direction = Direction::Left;
                        self.pos -= 1;
                    }
                    else
                    {
                        self.row += 1;
                    }
                }
                else
                {
                    self.row += 1;
                }
            },
            Direction::Left =>
            {
                if self.pos - 1 >= 0
                {
                    if board[self.row as usize][self.pos as usize - 1] == '#'
                    {
                        self.direction = Direction::Up;
                        self.row -= 1;
                    }
                    else
                    {
                        self.pos -= 1;
                    }
                }
                else
                {
                    self.pos -= 1;
                }
            },
            Direction::Right =>
            {
                if self.pos + 1 < (board[self.row as usize].len() as i32)
                {
                    if board[self.row as usize][self.pos as usize + 1] == '#'
                    {
                        self.direction = Direction::Down;
                        self.row += 1;
                    }
                    else
                    {
                        self.pos += 1;
                    }
                }
                else
                {
                    self.pos += 1;
                }
            }
        }
    }

    fn new_path(&self, board: &Vec<Vec<char>>) -> bool
    {
        if board[self.row as usize][self.pos as usize] == 'X'
        {
            return false;
        }

        return true;
    }
}

pub fn part1()
{
    let mut board: Vec<Vec<char>> = vec![];
    let mut guard: Guard = Guard { row: 0, pos: 0, direction: Direction::Up };

    if let Ok(lines) = utils::read_lines("src/day6/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let mut chars = l.chars().collect::<Vec<char>>();

                if chars.contains(&'^')
                {
                    guard.row = board.len() as i32;
                    guard.pos = chars.iter().position(|&x| x == '^').unwrap() as i32;
                    chars[guard.pos as usize] = 'X';
                }

                board.push(chars);

            }
        }
    }

    let mut distinct_positions: u32 = 1;

    while !guard.is_outside(&board)
    {
        guard.move_position(&mut board);

        if !guard.is_outside(&board) && guard.new_path(&board)
        {
            board[guard.row as usize][guard.pos as usize] = 'X';
            distinct_positions += 1;
        }
    }

    println!("Part 1: {}", distinct_positions);
}

pub fn part2()
{

}
