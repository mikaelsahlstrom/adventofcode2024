use std::vec;
use regex::Regex;
use rayon::prelude::*;

use crate::utils;

pub fn part1()
{
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut sum: u32 = 0;

    if let Ok(lines) = utils::read_lines("src/day1/input")
    {
        let mut left: Vec<u32> = vec![];
        let mut right: Vec<u32> = vec![];

        for line in lines
        {
            if let Ok(l) = line
            {
                let caps = re.captures(&l).unwrap();

                left.push(caps.get(1).unwrap().as_str().parse::<u32>().unwrap());
                right.push(caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
            }
        }

        left.par_sort_unstable_by(|a, b| b.cmp(a));
        right.par_sort_unstable_by(|a, b| b.cmp(a));

        while !left.is_empty()
        {
            sum += left.pop().unwrap().abs_diff(right.pop().unwrap());
        }

        println!("Part 1: {}", sum);
    }
}

pub fn part2()
{
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    if let Ok(lines) = utils::read_lines("src/day1/input")
    {
        let mut left: Vec<u32> = vec![];
        let mut right: Vec<u32> = vec![];

        for line in lines
        {
            if let Ok(l) = line
            {
                let caps = re.captures(&l).unwrap();

                left.push(caps.get(1).unwrap().as_str().parse::<u32>().unwrap());
                right.push(caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
            }
        }

        let score = left.par_iter().map(
            |l|
            {
                (*l as usize) * right.par_iter().filter(|r| *r == l).count()
            }
        ).sum::<usize>();

        println!("Part 2: {}", score);
    }
}
