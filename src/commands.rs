use std::{io::Result, process::Command};

/// Sends the work message to `terminal-notifier`.
pub fn work_msg(work_time_secs: u32) -> Result<()> {
    let msg = format!("Starting work timer ({work_time_secs:.0} minutes)");

    terminal_notifier(&msg)?;

    Ok(())
}

/// Sends the break message to `terminal-notifier`.
pub fn break_msg(break_time_secs: u32, num_loops: Option<usize>) -> Result<()> {
    let msg = format!(
        "Work over — take a break ({break_time_secs:.0} minutes{})",
        num_loops.map_or(String::new(), |loops| format!(", {loops} cycles remaining"))
    );

    terminal_notifier(&msg)?;

    Ok(())
}

/// Sends the finishing message to `terminal-notifier`.
pub fn over_msg() -> Result<()> {
    terminal_notifier("Timer is finished! Good job!")?;
    Ok(())
}

/// Sends the command to `terminal-notifier`.
fn terminal_notifier(msg: &str) -> Result<()> {
    Command::new("terminal-notifier")
        .args(["-title", "Pomodoro", "-message", msg, "-sound", "Purr"])
        .output()?;

    Ok(())
}
