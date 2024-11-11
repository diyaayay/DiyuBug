use std::io;
use std::io::Read;
use crossterm::terminal;

struct CleanUp;

impl Drop for CleanUp {
    fn drop (&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    let mut buf = [0; 1]; //read one byte at a time
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
        let character = buf[0] as char;
        if character.is_control() {
            println!("{}\r", character as u8) // is control char
        } else {
            println!("{}\r", character) // raw mode move cursor back to left
        }

    }
}