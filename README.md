# rusty_input
tui key tracker and logger created for X11 based linux  

### dependencies

> "xinput" command

### compilation and running

Download and compile with:

    git clone https://github.com/Creeper-boop/rusty_input.git  
    cd rusty_input  
    cargo build --release

to access and run the built binary with example tui:

    cd target/release/
    ./rusty_input -p ../../ExampleTui

### arguments

> -d or --debug for debug info  
> -l or --log for input log file or keylogger capabilities
> 
> - the log file is created in the binary working directory and always includes input device data  
> 
> -c or --compat to disable terminal raw mode  
> -p or --path to provide path to tui file

### hotkeys

> "ctrl + l" normally clearing the terminal emulator in raw mode reloads the tui  
> "ctrl + c" stops the process regardless of raw mode

### tui configuration

user-friendly tui editing is work in progress  
Tui data is stored in a text file loaded from the working directory or from the specified path.  
All tui strings can and should be formatted with ANSI escape sequences.

The file is structured to include all static formatted strings in the first line:

    \x1b[2J\x1b[38;2;128;128;128m\x1b[1;3H╔═══╗\x1b[2;3H║ A ║\x1b[3;3H╚═══╝\x1b[0m

with the line starting by clearing the screen and ending by resetting special parameters.  
Static data is followed by strings bound to keys:

    [13, 38]\x1b[2;4H\x1b[48;2;128;128;128m A \x1b[0m
    [14, 38]\x1b[2;4H A \x1b[0m

each key requiring two actions one for press and one for release events respectively.  
The key and event code [13, 38] representing "a" being pressed is followed by a formatted string that creates the 
desired outcome. The second or the release action should return the changed part of tui to its "default" state. 
Key codes and event numbers can be gathered using debug mode.  
Action data ends with an empty line or end of file.
