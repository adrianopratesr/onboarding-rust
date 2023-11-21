mod terminal;
mod todo;

use std::io::Result;
use terminal::Terminal;
use todo::Todo;

fn main() -> Result<()> {
    let terminal = Terminal::new();
    loop {
        terminal.ask_if_new_todo()?;
        let new_todo = terminal.ask_for_new_todo()?;
        terminal.show_todo(&new_todo)?;
    }
}
