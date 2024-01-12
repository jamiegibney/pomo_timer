use super::*;
use std::process::Command;

#[test]
#[should_panic]
#[allow(clippy::should_panic_without_expect)]
fn bad_terminal_notifier_path() {
    assert!(Command::new("which")
        .args(["terminal-notifierrr"])
        .output()
        .map_or(false, |output| output.status.success()));
}

#[test]
fn try_terminal_notifier() {
    Command::new("terminal-notifier")
        .args([
            "-title",
            "Test title",
            "-message",
            "test message",
            "-sound",
            "Purr",
        ])
        .output()
        .unwrap();
}

#[test]
fn handle_args_zero() {
    let pomo = Pomo {
        work_time: 0,
        break_time: 0,
        num_loops: 0,
    };

    pomo.run().unwrap();
}

#[test]
fn parse_help_message() {
    // simulate arguments passed to the program
    let mut args = ["pomo", "help", "36fh78adf8hf8"].into_iter().skip(1);
    assert!(contains_help(args.next().unwrap()));

    // simulate typo
    let mut args = ["pomo", "hhelp"].into_iter().skip(1);
    assert!(contains_help(args.next().unwrap()));

    // simulate wrong command
    let mut args = ["pomo", "ehlp"].into_iter().skip(1);
    assert!(!contains_help(args.next().unwrap()));
}
