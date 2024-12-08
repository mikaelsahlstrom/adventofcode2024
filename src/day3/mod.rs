use crate::utils;

pub fn part1()
{
    let mut sum: u32 = 0;

    if let Ok(lines) = utils::read_lines("src/day3/input")
    {
        let mut line: String = match lines.collect()
        {
            Ok(l) => l,
            Err(_) => return
        };

        let re = regex::Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        while !line.is_empty()
        {
            if re.is_match(&line)
            {
                let caps = re.captures(&line).unwrap();
                let a: u32 = caps[1].parse().unwrap();
                let b: u32 = caps[2].parse().unwrap();
                sum += a * b;
            }

            line.remove(0);
        }
    }

    println!("Part 1: {}", sum);
}

pub fn part2()
{

}
