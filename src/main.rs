use clap::Parser;

mod utils;
mod day1;

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
        "all" | &_ =>
        {
            println!("Day 1");
            day1::part1();
            day1::part2();
        }
    }
}
