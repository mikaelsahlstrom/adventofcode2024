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

fn is_safe(mut parts: Vec<&str>) -> bool
{
    let mut first = parts.pop().unwrap().parse::<u32>().unwrap();
    let mut second = parts.pop().unwrap().parse::<u32>().unwrap();

    if !safe_differ(first, second) || first == second
    {
        return false;
    }

    // Calculate direction.
    let direction = if first > second { Direction::Down } else { Direction::Up };

    while !parts.is_empty()
    {
        first = second;
        second = parts.pop().unwrap().parse::<u32>().unwrap();

        if !safe_differ(first, second) || first == second
        {
            return false;
        }

        match direction
        {
            Direction::Up =>
            {
                if first > second
                {
                    return false;
                }
            },
            Direction::Down =>
            {
                if first < second
                {
                    return false;
                }
            }
        }
    }

    return true;
}

pub fn part2()
{
    let mut count: u32 = 0;

    if let Ok(lines) = utils::read_lines("src/day2/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let parts = l.split_whitespace().collect::<Vec<&str>>();

                if is_safe(parts.clone())
                {
                    count += 1;
                }
                else
                {
                    for i in 0..parts.len()
                    {
                        let mut new_parts = parts.clone();
                        new_parts.remove(i);
                        if is_safe(new_parts)
                        {
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", count);
}
