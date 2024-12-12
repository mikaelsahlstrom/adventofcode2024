use clap::Parser;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args
{
    #[arg(short, long, default_value = "all")]
    day: String,
    #[arg(short, long, default_value = "all")]
    part: String
}

fn main() {
    let args = Args::parse();

    match args.day.as_str()
    {
        "1" =>
        {
            println!("Day 1");
            match args.part.as_str()
            {
                "1" =>
                {
                    day1::part1();
                },
                "2" =>
                {
                    day1::part2();
                },
                "all" | &_ =>
                {
                    day1::part1();
                    day1::part2();
                }
            }
        },
        "2" =>
        {
            println!("Day 2");
            match args.part.as_str()
            {
                "1" =>
                {
                    day2::part1();
                },
                "2" =>
                {
                    day2::part2();
                },
                "all" | &_ =>
                {
                    day2::part1();
                    day2::part2();
                }
            }
        },
        "3" =>
        {
            println!("Day 3");
            match args.part.as_str()
            {
                "1" =>
                {
                    day3::part1();
                },
                "2" =>
                {
                    day3::part2();
                },
                "all" | &_ =>
                {
                    day3::part1();
                    day3::part2();
                }
            }
        },
        "4" =>
        {
            println!("Day 4");
            match args.part.as_str()
            {
                "1" =>
                {
                    day4::part1();
                },
                "2" =>
                {
                    day4::part2();
                },
                "all" | &_ =>
                {
                    day4::part1();
                    day4::part2();
                }
            }
        },
        "all" | &_ =>
        {
            println!("Day 1");
            day1::part1();
            day1::part2();

            println!("Day 2");
            day2::part1();
            day2::part2();

            println!("Day 3");
            day3::part1();
            day3::part2();

            println!("Day 4");
            day4::part1();
            day4::part2();
        }
    }
}
