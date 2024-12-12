use crate::utils;

fn right(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if pos + find.len() > board[row].len()
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row][pos + i] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn left(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if (pos as i32) - ((find.len() as i32) - 1) < 0
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row][pos - i] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn up(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if (row as i32) - ((find.len() as i32) - 1) < 0
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row - i][pos] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn down(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if row + find.len() > board.len()
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row + i][pos] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn up_right(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if (row as i32) - ((find.len() as i32) - 1) < 0 || pos + find.len() > board[row].len()
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row - i][pos + i] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn up_left(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if (row as i32) - ((find.len() as i32) - 1) < 0 || (pos as i32) - ((find.len() as i32) - 1) < 0
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row - i][pos - i] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn down_right(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if row + find.len() > board.len() || pos + find.len() > board[row].len()
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row + i][pos + i] != find[i]
        {
            return false;
        }
    }

    return true;
}

fn down_left(board: &Vec<Vec<char>>, row: usize, pos: usize) -> bool
{
    let find = vec!['X', 'M', 'A', 'S'];

    if row + find.len() > board.len() || (pos as i32) - ((find.len() as i32) - 1) < 0
    {
        return false;
    }

    for i in 0..find.len()
    {
        if board[row + i][pos - i] != find[i]
        {
            return false;
        }
    }

    return true;
}

pub fn part1()
{
    let mut board: Vec<Vec<char>> = vec![];

    if let Ok(lines) = utils::read_lines("src/day4/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let mut row: Vec<char> = vec![];

                for c in l.chars()
                {
                    row.push(c);
                }

                board.push(row);
            }
        }
    }

    let mut count: u32 = 0;

    for i in 0..board.len()
    {
        for j in 0..board[i].len()
        {
            if right(&board, i, j)
            {
                count += 1;
            }

            if left(&board, i, j)
            {
                count += 1;
            }

            if up(&board, i, j)
            {
                count += 1;
            }

            if down(&board, i, j)
            {
                count += 1;
            }

            if up_right(&board, i, j)
            {
                count += 1;
            }

            if up_left(&board, i, j)
            {
                count += 1;
            }

            if down_right(&board, i, j)
            {
                count += 1;
            }

            if down_left(&board, i, j)
            {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}

pub fn part2()
{

}
