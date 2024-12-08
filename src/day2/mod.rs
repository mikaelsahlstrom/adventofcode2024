use crate::utils;

enum Direction
{
    Up,
    Down
}

fn safe_differ(a: u32, b: u32) -> bool
{
    let diff = a.abs_diff(b);
    return diff >= 1 && diff <= 3;
}

pub fn part1()
{
    let mut count: u32 = 0;

    if let Ok(lines) = utils::read_lines("src/day2/input")
    {
        'line: for line in lines
        {
            if let Ok(l) = line
            {
                let mut parts = l.split_whitespace().collect::<Vec<&str>>();

                let mut first = parts.pop().unwrap().parse::<u32>().unwrap();
                let mut second = parts.pop().unwrap().parse::<u32>().unwrap();

                if !safe_differ(first, second) || first == second
                {
                    continue 'line;
                }

                // Calculate direction.
                let direction = if first > second { Direction::Down } else { Direction::Up };

                while !parts.is_empty()
                {
                    first = second;
                    second = parts.pop().unwrap().parse::<u32>().unwrap();

                    if !safe_differ(first, second) || first == second
                    {
                        continue 'line;
                    }

                    match direction
                    {
                        Direction::Up =>
                        {
                            if first > second
                            {
                                continue 'line;
                            }
                        },
                        Direction::Down =>
                        {
                            if first < second
                            {
                                continue 'line;
                            }
                        }
                    }
                }

                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}
