mod commands;
use commands::{break_msg, over_msg, work_msg};
use std::{env, io::Result, process::exit, thread::sleep, time::Duration};

/// Default work time of 25 minutes.
const DEFAULT_WORK_TIME: u32 = 25;
/// Default break time of 5 minutes.
const DEFAULT_SLEEP_TIME: u32 = 5;
/// Message printed if "help" is passed as an argument.
const HELP_MESSAGE: &str = include_str!("../help_message.txt");

fn main() {
    // obtain data from command line args
    let pomo = Pomo::new();
    pomo.print_start_message();

    // run the timer.
    match pomo.run() {
        Ok(()) => {
            // if the finish notification fails
            if let Err(e) = over_msg() {
                eprintln!(
                    "Pomodoro timer failed to print finished method: {e}"
                );
                exit(1);
            }
        }
        Err(e) => {
            // the timer failed somewhere (i.e. terminal-notifier is not found)
            eprintln!("Pomodoro timer encountered an error: {e}");
            eprintln!("Do you have `terminal-notifier` installed?");
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
    pub num_loops: Option<usize>,
}

impl Pomo {
    /// Captures the necessary state from the command line args.
    pub fn new() -> Self {
        let mut args = env::args().skip(1);

        // TODO: if a single, numeric argument is passed, use it for the number of 
        // loops with the default timings?
        let one_arg_passed = args.len() == 1;

        // obtain the work time, if provided
        let work_time = args.next().map_or(DEFAULT_WORK_TIME, |v| {
            v.parse::<u32>().map_or_else(
                |_| {
                    // if "help" detected, print the help message
                    if v.to_lowercase().contains("help") {
                        print_help_message();
                        exit(0);
                    }

                    DEFAULT_WORK_TIME
                },
                |value| value,
            )
        });

        // obtain the work time, if provided
        let break_time = args.next().map_or_else(|| {
            // warn if only one (non-"help") argument was passed, as this may seem ambiguous
            if one_arg_passed {
                println!("WARNING: only one argument passed, using default break timer of {DEFAULT_SLEEP_TIME} minutes"); 
            }

            DEFAULT_SLEEP_TIME 
        }, |v| {
            v.parse::<u32>().map_or(DEFAULT_SLEEP_TIME, |value| value)
        });

        // obtain the number of loops, if provided. else set to None, which will
        // run an endless loop.
        let num_loops: Option<usize> =
            args.next().and_then(|v| v.parse::<usize>().ok());

        Self { work_time, break_time, num_loops }
    }

    /// Prints the initial starting message to `stdout`.
    pub fn print_start_message(&self) {
        let msg = format!(
            "Running Pomodoro timer: work {} mins, break {} mins",
            self.work_time, self.break_time
        );

        println!(
            "{msg}{}",
            self.num_loops
                .as_ref()
                .map_or_else(String::new, |num_loops| format!(
                    ", {num_loops} times"
                ))
        );
    }

    /// Runs the timer.
    pub fn run(&self) -> Result<()> {
        // if a number of loops is provided...
        if let Some(num_loops) = self.num_loops {
            let mut counter = num_loops;

            while counter > 0 {
                counter -= 1;

                work_msg(self.work_time)?;
                sleep_for_seconds(self.work_time * 60);

                // we don't need to send a message or sleep for the break if this is 
                // the final cycle.
                if counter == 0 {
                    break;
                }

                break_msg(self.break_time, Some(counter))?;
                sleep_for_seconds(self.break_time * 60);
            }

            return Ok(());
        }

        // otherwise, we run an endless loop!
        loop {
            work_msg(self.work_time)?;
            sleep_for_seconds(self.work_time * 60);

            break_msg(self.break_time, None)?;
            sleep_for_seconds(self.break_time * 60);
        }
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
