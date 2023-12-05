use structopt::StructOpt;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(StructOpt)]
struct Cli {
    day: u16,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        5 => day05::day05(),
        6 => day06::day06(),
        7 => day07::day07(),
        8 => day08::day08(),
        9 => day09::day09(),
        10 => day10::day10(),
        11 => day11::day11(),
        12 => day12::day12(),
        13 => day13::day13(),
        14 => day14::day14(),
        15 => day15::day15(),
        16 => day16::day16(),
        17 => day17::day17(),
        18 => day18::day18(),
        19 => day19::day19(),
        20 => day20::day20(),
        21 => day21::day21(),
        22 => day22::day22(),
        23 => day23::day23(),
        24 => day24::day24(),
        25 => day25::day25(),
        411 => {
            day01::day01();
            day02::day02();
            day03::day03();
            day04::day04();
            day05::day05();
            day06::day06();
            day07::day07();
            day08::day08();
            day09::day09();
            day10::day10();
            day11::day11();
            day12::day12();
            day13::day13();
            day14::day14();
            day15::day15();
            day16::day16();
            day17::day17();
            day18::day18();
            day19::day19();
            day20::day20();
            day21::day21();
            day22::day22();
            day23::day23();
            day24::day24();
            day25::day25();
        }
        _ => println!("Unimplemented day: {}", args.day),
    }
}
