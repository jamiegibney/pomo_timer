# `pomo`
Basic Pomodoro timer for the command line which uses `terminal-notifier`.

### Requires [terminal-notifier](https://github.com/julienXX/terminal-notifier).

# Usage
To see the valid usage, run:
```bash
pomo help
```

To endlessly run a 25/5 Pomodoro cycle, run:
```bash
pomo
```

To set a custom work/break time (in minutes), you may run something like:
```bash
pomo 60 15
```

A third argument will run a certain number of cycles. E.g., for 5 cycles, run:
```bash
pomo 25 5 5
```

# TODO
- [x] Allow a single numeric argument to define the number of cycles with the default timer settings
- [x] Add a default cycle count, rather than it being endless.
- [ ] Remove endless cycle count, and use fall back to the default.
- [ ] Drop requirement for `terminal-notifier`, either by including notification functionality in this project or, ideally, by not crashing
- [ ] Are there methods of running the timer in parallel that can be included here directly?
