use std::{env, io::Error};

use solutions::{Solver, day1::Day1};

pub mod solutions;

#[derive(Debug)]
enum ProcessNameError {
    NoFile,
    NotUtf8,
    Io(Error)
}

impl From<Error> for ProcessNameError {
    fn from(err: Error) -> Self {
        ProcessNameError::Io(err)
    }
}

fn get_process_name() -> Result<String, ProcessNameError> {
    Ok(env::current_exe()?
        .file_name().ok_or(ProcessNameError::NoFile)?
        .to_str().ok_or(ProcessNameError::NotUtf8)?
        .to_owned()
    )
}

fn print_usage() {
    println!("Usage: {} <all|1-25> [repeat_count]", get_process_name().unwrap());
}

fn run_solver(day: u8, repeat_count: u32) {
    match day {
        1 => Day1::run(repeat_count),
        2..=25 => (),
        _ => unreachable!()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() >= 4 {
        return print_usage();
    }
    
    let mut repeat_count: u32 = 1;
    if args.len() >= 3 {
        if let Ok(parsed_repeat_count) = args[2].parse::<u32>() {
            repeat_count = parsed_repeat_count;
        } else {
            return print_usage();
        }
    }

    if args[1] == "all" {
        for day in 1..=25 {
            run_solver(day, repeat_count);
        }
    } else if let Ok(day) = args[1].parse::<u8>() {
        if day < 1 || day > 25 {
            return print_usage();
        }
        run_solver(day, repeat_count);
    } else {
        return print_usage();
    }
}