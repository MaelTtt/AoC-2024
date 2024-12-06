use std::{env, fs, process, time};

use aoc_2024::AoCResult;

struct Config {
    day: u8,
    input_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut use_input_file = false;
        let mut day: Option<&String> = None;
        let mut input_path: Option<&String> = None;

        for arg in args.into_iter().skip(1) {
            if arg == "-t" {
                use_input_file = true;
            } else if day.is_none() {
                day = Some(arg);
            } else {
                input_path = Some(arg);
            }
        }

        let day = match day {
            None => return Err("Day not provided"),
            Some(s) => match s.parse::<u8>() {
                Ok(parsed_day) => parsed_day,
                Err(_) => return Err("Invalid Day Format"),
            },
        };

        let input_path = match input_path {
            None => format!(
                "data/input{}{}",
                day,
                if use_input_file { "Test" } else { "" }
            ),
            Some(s) => s.clone(),
        };

        Ok(Config { day, input_path })
    }
}

macro_rules! expand_days {
    ( $( $day_num:expr => $mod:ident ),* ) => {
        $(
            mod $mod;
        )*

        fn run_day_part(fn_part: fn (&str) -> AoCResult, input: &str, desc: &str) {
            let start = time::Instant::now();
            let res = fn_part(input);
            let time = start.elapsed();
            print!("{desc} ({time:?}): ");
            match res {
                AoCResult::None => {println!("No value")},
                AoCResult::Str(val) => {println!("{val}")},
                AoCResult::Int(val) => {println!("{val}")}
            }
        }

        fn run_day(day: u8, input: &str) {
            match day {
                $(
                    $day_num => {
                        println!("Day {day}");
                        run_day_part($mod::part_one, &input, "Part one");
                        run_day_part($mod::part_two, &input, "Part two");
                    },
                )*
                _ => {
                    println!("Day {day} not implemented.");
                }
            }
        }
    };
}

expand_days!(1 => day1);

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let input = fs::read_to_string(&config.input_path)
        .expect(&format!("Couldn't read file: {0}", config.input_path));

    run_day(config.day, &input);
}
