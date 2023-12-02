use structopt::StructOpt;
mod day01;
mod day02;

#[derive(StructOpt)]
struct Cli {
    day: u16,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day01::day01(),
        2 => day02::day02(),
        411 => {
            day01::day01();
            day02::day02();
        }
        _ => println!("Unimplemented day: {}", args.day),
    }
}
