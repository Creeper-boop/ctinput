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
> -p or --path to provide path to tui file defaults to "Tui"  
> -r or --runners to enable runners

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
Static data is followed by reactive and scripted elements or runners.  
Reactive elements are strings bound to keys formatted:

    [13, 38]\x1b[2;4H\x1b[48;2;128;128;128m A \x1b[0m
    [14, 38]\x1b[2;4H A \x1b[0m

each key requiring two actions one for press and one for release events respectively.  
The key and event code [13, 38] representing "a" being pressed is followed by a formatted string that creates the 
desired outcome. The second or the release action should return the changed part of tui to its "default" state. 
Key codes and event numbers can be gathered using debug mode.  
Scripted elements are defined by their name and required data separated by whitespaces:  

    history 24 0 10 16
    apm 10 6 13

Above being examples of implemented scripted elements.  
Runners are similar to macros being able to bind commands to events:  

    runner [13, 24] ACTIVE:\x1b[0;7H\x1b[48;2;255;0;0m Q \x1b[0m INACTIVE:\x1b[0;7H   \x1b[0m COMMAND:sleep 5s

Above being an example of a runner that binds the "sleep 5s" command to q. 
With "ACTIVE:" and "INACTIVE:" fields being tui updates when the command begins and ends execution.
When formatting all field names must be preceded with a whitespace.  
Action data ends with an empty line or end of file.
