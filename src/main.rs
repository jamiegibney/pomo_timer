use std::{env, io::Result, process::exit, thread::sleep, time::Duration};

mod commands;
#[cfg(test)]
mod tests;
use commands::{break_msg, over_msg, terminal_notifier_located, work_msg};

/// Default work time of 25 minutes.
const DEFAULT_WORK_TIME: u32 = 25;
/// Default break time of 5 minutes.
const DEFAULT_BREAK_TIME: u32 = 5;
/// Default number of cycles.
const DEFAULT_NUM_CYCLES: usize = 5;
/// Message printed if "help" is passed as an argument.
const HELP_MESSAGE: &str = include_str!("help_message.txt");

fn main() {
    // check for the presence of terminal-notifier
    if !terminal_notifier_located() {
        eprintln!("ERROR: failed to locate terminal-notifier. Please install it here:\nhttps://github.com/julienXX/terminal-notifier");
        exit(1);
    }

    // obtain data from command line args
    let pomo = Pomo::new();
    pomo.print_start_message();

    // run the timer.
    match pomo.run() {
        Ok(()) => {
            // if the finish notification fails
            if let Err(e) = over_msg() {
                eprintln!(
                    "Pomodoro timer failed to print finished message: {e}"
                );
                exit(1);
            }
        }
        Err(e) => {
            // the timer failed somewhere (i.e. terminal-notifier is not found)
            eprintln!("Pomodoro timer encountered an error:\n{e}");
            exit(1);
        }
    };

    exit(0);
}

/// Timer state.
struct Pomo {
    /// The time (in minutes) of the work cycle.
    pub work_time: u32,
    /// The time (in minutes) of the break cycle.
    pub break_time: u32,
    /// The optional number of timer loops.
    pub num_loops: usize,
}

impl Pomo {
    /// Captures the necessary state from the command line args.
    pub fn new() -> Self {
        // check to see if help is present
        for arg in env::args() {
            if contains_help(&arg) {
                print_help_message();
                exit(0);
            }
        }

        let mut args = env::args().skip(1);
        let mut num_loops = DEFAULT_NUM_CYCLES;

        if args.len() == 1 {
            // this will not panic as we've already check for the number of arguments.
            let arg = args.next().unwrap();

            // if the argument is valid
            if arg.chars().all(|ch| ch.is_numeric()) {
                if let Ok(value) = arg.parse() {
                    num_loops = value;
                }
            }
            else {
                eprintln!("ERROR: unknown argument \"{arg}\"");
                exit(1);
            }
        }

        // obtain the work time, if provided
        let work_time = args.next().map_or(DEFAULT_WORK_TIME, |v| {
            v.parse::<u32>()
                .map_or_else(|_| {
                    eprintln!("WARNING: unknown argument {v}; using default work time of {DEFAULT_WORK_TIME} minutes");
                    DEFAULT_WORK_TIME
                }
                             , |value| value)
        });

        // obtain the work time, if provided
        let break_time = args.next().map_or_else(
            || DEFAULT_BREAK_TIME,
            |v| v.parse::<u32>().map_or_else(|_| {
                    eprintln!("WARNING: unknown argument {v}; using default break time of {DEFAULT_BREAK_TIME} minutes");
                DEFAULT_BREAK_TIME
            }, |value| value),
        );

        // obtain the number of loops, if provided
        if let Some(arg_3) = args.next() {
            arg_3.parse().map_or_else(|_| {
                eprintln!("WARNING: unknown argument {arg_3}; using default cycle count of {DEFAULT_NUM_CYCLES}");
            }, |value| {
                num_loops = value;
            });
        }

        Self { work_time, break_time, num_loops }
    }

    /// Prints the initial starting message to `stdout`.
    pub fn print_start_message(&self) {
        let msg = format!(
            "Running Pomodoro timer: work {} {}, break {} {}",
            self.work_time,
            parse_plural(self.work_time, "min"),
            self.break_time,
            parse_plural(self.break_time, "min"),
        );

        println!(
            "{msg}, {} {}",
            self.num_loops,
            parse_plural(self.num_loops as u32, "cycle"),
        );
    }

    /// Runs the timer.
    pub fn run(&self) -> Result<()> {
        let mut counter = self.num_loops;

        while counter > 0 {
            counter -= 1;

            work_msg(self.work_time)?;
            sleep_for_seconds(self.work_time * 60);

            // we don't need to send a message or sleep for the break if this is
            // the final cycle.
            if counter == 0 {
                break;
            }

            break_msg(self.break_time, counter)?;
            sleep_for_seconds(self.break_time * 60);
        }

        Ok(())
    }
}

/// Sleeps the current thread for `time_secs` seconds.
fn sleep_for_seconds(time_secs: u32) {
    sleep(Duration::from_secs(time_secs as u64));
}

/// Prints the help message to `stdout`.
fn print_help_message() {
    print!("{HELP_MESSAGE}");
}

/// Parses a string to deal with plural values (only appends an "s" if the value > 1).
fn parse_plural(value: u32, message: &str) -> String {
    format!("{message}{}", if value > 1 { "s" } else { "" })
}

/// Returns `true` if `arg` contains "help".
fn contains_help(arg: &str) -> bool {
    arg.to_lowercase().contains("help")
}
