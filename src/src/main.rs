use termcolor::{Color, ColorChoice, Color, ColorSpec, WriteColor};

// General approach:
// Daemon, clients communicate via named pipe keyed to process
//  - Single daemon per user
//  - Pipes should only be read/writable by creating user
//  - Daemon acts as cache for info from git, maybe other things in future?
//    - Ssh status?
// Configured w/ zsh
// Communication:
//  - Client checks if daemon exists, spawn if no
//  - Client sends current directory to initialize every communication
//     - Also any other state that could need tracking, but I *think* that should be enough
// Client responsibilities:
//  - Keep track of terminal size, figure out how to smoosh stuff if required
//  - Rendering
//  - Get git info from daemon
//    - Branch, fetch status, pull status, dirty status, stash
//  - Display background processes, potentially w/ names and pid
//  - Long running process time
//  - Ssh status?
//  - Does the client have extended fonts for fancy characters?
//
//  - Nix shell
//  - Python venv
//  - Fire off dbus message if real long process finishes
//
//  Implementaiton notes:
//   - Available hooks from zsh:
//     http://zsh.sourceforge.net/Doc/Release/Functions.html#Hook-Functions

fn main() {
    println!("Hello, world!");
}
