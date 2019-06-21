extern crate ncurses;
use ncurses::*;

mod command_input;
mod source_list;
use command_input::CommandInput;
use source_list::SourceList;

const ADD_SOURCE_KEY: i32 = 105; // 'i' key

struct App {
    source_list: SourceList,
    command_input: CommandInput,
}

impl App {
    pub fn new() -> App {
        let source_list = SourceList::new();
        let command_input = CommandInput::new();
        App {
            source_list: source_list,
            command_input: command_input,
        }
    }

    pub fn start(&mut self) {
        self.source_list.render();
        self.command_input.render();
        loop {
            let ch: i32 = getch();
            match ch {
                ADD_SOURCE_KEY => self.command_input.prompt("Add source: "),
                _ => (),
            };
        }
    }
}

fn main() {
    initscr();
    noecho();
    refresh();
    let mut app = App::new();
    app.start();
}
