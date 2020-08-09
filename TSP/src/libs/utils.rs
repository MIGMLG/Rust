use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{stdin, stdout, Write};

//Function to clean the terminal screen
pub fn clear_screen() {
    if cfg!(windows) {
        execute!(stdout(), Clear(ClearType::All));
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

//Function to read the input
pub fn read_input() -> String {
    let mut s = String::new();

    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}