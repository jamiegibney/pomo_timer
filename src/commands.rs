use super::parse_plural;
use std::{io::Result, process::Command};

/// Sends the work message to `terminal-notifier`.
pub fn work_msg(work_time_mins: u32) -> Result<()> {
    let msg = format!(
        "Starting work timer ({work_time_mins:.0} {})",
        parse_plural(work_time_mins, "minute")
    );

    terminal_notifier(&msg)?;

    Ok(())
}

/// Sends the break message to `terminal-notifier`.
pub fn break_msg(break_time_mins: u32, num_loops: usize) -> Result<()> {
    let msg = format!(
        "Work over — take a break ({break_time_mins:.0} {}, {} {} remaining)",
        parse_plural(break_time_mins, "minute"),
        num_loops,
        parse_plural(num_loops as u32, "cycle"),
    );

    terminal_notifier(&msg)?;

    Ok(())
}

/// Sends the finishing message to `terminal-notifier`.
pub fn over_msg() -> Result<()> {
    terminal_notifier("Timer is finished! Good job!")?;
    Ok(())
}

/// Returns true if `terminal-notifier` is found.
pub fn terminal_notifier_located() -> bool {
    Command::new("which")
        .args(["terminal-notifier"])
        .output()
        .map_or(false, |output| output.status.success())
}

/// Sends the command to `terminal-notifier`.
fn terminal_notifier(msg: &str) -> Result<()> {
    Command::new("terminal-notifier")
        .args(["-title", "Pomodoro", "-message", msg, "-sound", "Purr"])
        .output()?;

    Ok(())
}
