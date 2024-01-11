# Overview
Basic Pomodoro timer for the command line on macOS which uses `terminal-notifier`.

**macOS only!**

### Requires [terminal-notifier](https://github.com/julienXX/terminal-notifier).

# Usage
```
$ pomo help
usage: 
    pomo
    pomo [number of cycles]
    pomo [work time (minutes)] [break time (minutes)]
    pomo [work time (minutes)] [break time (minutes)] [number of cycles]
examples: 
    `pomo`         -> 25 min work, 5 min break,  5 cycles (default)
    `pomo 1`       -> 25 min work, 5 min break,  1 cycle
    `pomo 60 15`   -> 60 min work, 15 min break, 5 cycles 
    `pomo 60 15 3` -> 60 min work, 15 min break, 3 cycles
```

# Installing

Pre-built binaries are available on the [releases](https://github.com/jamiegibney/pomo_timer/releases) page.

After downloading, run this command to create a symlink so you can run the command anywhere:
```bash
ln -si path/to/downloaded/file /usr/local/bin/pomo
```

#### Installing from source

Simply clone the repository and run [`install.sh`](./install.sh), as below. Requires Rust `1.75.0` or above.
```bash
$ git clone https://github.com/jamiegibney/pomo_timer.git
$ cd pomo_timer
$ ./install.sh
```

# TODO
- [x] Allow a single numeric argument to define the number of cycles with the default timer settings
- [x] Add a default cycle count, rather than it being endless.
- [x] Remove endless cycle count, and use fall back to the default.
- [ ] Drop requirement for `terminal-notifier`, either by including notification functionality in this project or, ideally, by not crashing
- [ ] Are there methods of running the timer in parallel that can be included here directly?
