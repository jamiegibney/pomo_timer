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

</br>

To send the timer to the background, you may append `&` to the command. For example:
```bash
pomo 60 15 3 &
```

</br>

To stop the timer, you can use the `fg` command to send `pomo` back to the foreground, and then exit the process with `ctrl-c`, for example.

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
