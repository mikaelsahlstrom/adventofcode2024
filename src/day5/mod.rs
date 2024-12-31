use std::collections::HashMap;

use crate::utils;

enum ParseState
{
    Before,
    Updates
}

pub fn part1()
{
    let mut parse_state = ParseState::Before;
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];

    if let Ok(lines) = utils::read_lines("src/day5/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                match parse_state
                {
                    ParseState::Before =>
                    {
                        if l.is_empty()
                        {
                            parse_state = ParseState::Updates;
                        }
                        else
                        {
                            let mut parts = l.split("|");
                            let key = parts.next().unwrap().parse::<u32>().unwrap();
                            let value = parts.next().unwrap().parse::<u32>().unwrap();

                            if !before.contains_key(&key)
                            {
                                before.insert(key, vec![]);
                            }

                            before.get_mut(&key).unwrap().push(value);
                        }
                    },
                    ParseState::Updates =>
                    {
                        updates.push(l.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
                    }
                }
            }
        }
    }

    let mut sum: u32 = 0;

    for i in 0..updates.len()
    {
        let mut correct = true;

        'outer: for j in 0..updates[i].len()
        {
            for k in 0..j
            {
                match before.get(&updates[i][j])
                {
                    Some(v) =>
                    {
                        if v.contains(&updates[i][k])
                        {
                            correct = false;
                            break 'outer;
                        }
                    },
                    None => {}
                }
            }
        }

        if correct
        {
            let middle_update = updates[i].len() / 2;
            sum += updates[i][middle_update];
        }
    }

    println!("Part 1: {}", sum);
}

fn reorder(update: &mut Vec<u32>, before: &HashMap<u32, Vec<u32>>)
{
    let mut new_order: Vec<u32> = vec![];

    for i in 0..update.len()
    {
        let mut inserted = false;

        for j in 0..new_order.len()
        {
            match before.get(&update[i])
            {
                Some(v) =>
                {
                    if v.contains(&new_order[j])
                    {
                        new_order.insert(j, update[i]);
                        inserted = true;
                        break;
                    }
                },
                None => {}
            }
        }

        if !inserted
        {
            new_order.push(update[i]);
        }
    }

    *update = new_order;
}

pub fn part2()
{
    let mut parse_state = ParseState::Before;
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];

    if let Ok(lines) = utils::read_lines("src/day5/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                match parse_state
                {
                    ParseState::Before =>
                    {
                        if l.is_empty()
                        {
                            parse_state = ParseState::Updates;
                        }
                        else
                        {
                            let mut parts = l.split("|");
                            let key = parts.next().unwrap().parse::<u32>().unwrap();
                            let value = parts.next().unwrap().parse::<u32>().unwrap();

                            if !before.contains_key(&key)
                            {
                                before.insert(key, vec![]);
                            }

                            before.get_mut(&key).unwrap().push(value);
                        }
                    },
                    ParseState::Updates =>
                    {
                        updates.push(l.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
                    }
                }
            }
        }
    }

    let mut sum: u32 = 0;

    for i in 0..updates.len()
    {
        let mut correct = true;

        'outer: for j in 0..updates[i].len()
        {
            for k in 0..j
            {
                match before.get(&updates[i][j])
                {
                    Some(v) =>
                    {
                        if v.contains(&updates[i][k])
                        {
                            correct = false;
                            break 'outer;
                        }
                    },
                    None => {}
                }
            }
        }

        if !correct
        {
            reorder(&mut updates[i], &before);

            let middle_update = updates[i].len() / 2;
            sum += updates[i][middle_update];
        }
    }

    println!("Part 2: {}", sum);
}
