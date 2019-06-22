extern crate feed_parser;
extern crate ncurses;

use ncurses::*;
mod command_input;
mod source_list;

use command_input::Command;
use command_input::CommandInput;
use source_list::SourceList;

use std::rc::Rc;
use std::cell::RefCell;

const ADD_SOURCE_KEY: i32 = 105; // 'i' key
const FOCUS_SOURCE_LIST_KEY: i32 = 108; // 'l' key
const ACTION_ADD_SOURCE: &'static str = "add_source";

struct App {
  source_list: Rc<RefCell<SourceList>>,
  command_input: CommandInput,
}

impl App {
  pub fn new() -> App {
    let source_list = Rc::new(RefCell::new(SourceList::new()));
    let command_input = CommandInput::new();
    App {
      source_list: source_list,
      command_input: command_input,
    }
  }
  pub fn start(&mut self) {
    self.source_list.borrow().render();
    self.command_input.render();
    let source_list_clone = self.source_list.clone();
    self.command_input.on_command(move |command: Command| {
      match command.name {
        ACTION_ADD_SOURCE => source_list_clone.borrow_mut().add_source(command.value),
        _ => (),
      };
    });
    loop {
      let ch: i32 = getch();
      match ch {
        ADD_SOURCE_KEY => {
          self.command_input.set_command_name(ACTION_ADD_SOURCE);
          self.command_input.prompt("Add source: ");
        }
        FOCUS_SOURCE_LIST_KEY => {
          self.source_list.borrow_mut().handle_focus();
        }
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
