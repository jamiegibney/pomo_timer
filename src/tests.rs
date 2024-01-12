use super::*;
use std::{io::Result, process::Command};

#[test]
#[should_panic(expected = "explicit panic")]
fn bad_terminal_notifier_path() {
    if !Command::new("which")
        .args(["terminal-notifierrr"])
        .output()
        .map_or(false, |output| output.status.success())
    {
        panic!();
    }
}

/// Sends the command to `terminal-notifier`.
#[test]
fn try_terminal_notifier() {
    Command::new("terminal-notifier")
        .args([
            "-title", "Test title", "-message", "test message", "-sound",
            "Purr",
        ])
        .output()
        .unwrap();
}

#[test]
fn handle_args_zero() {
    let pomo = Pomo { work_time: 0, break_time: 0, num_loops: 0 };

    pomo.run().unwrap();
}

#[test]
fn parse_help_message() {
    // simulate the arguments passed to the program
    let mut args = ["pomo", "help", "36fh78adf8hf8"].into_iter().skip(1);

    assert!(contains_help(args.next().unwrap()));
}
