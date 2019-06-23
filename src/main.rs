extern crate ncurses;
extern crate readability;

mod article_viewer;
mod command_input;
mod feed;
mod main_area;
mod source_list;
mod util;

use ncurses::*;

use command_input::Command;
use command_input::CommandInput;

use main_area::MainArea;
use source_list::{Source, SourceList};

use std::cell::RefCell;
use std::rc::Rc;

const ADD_SOURCE_KEY: i32 = 105; // 'i' key
const FOCUS_SOURCE_LIST_KEY: i32 = 108; // 'l' key
const FOCUS_FEED_KEY: i32 = 102; // 'f' key
const BACK_TO_FEED_KEY: i32 = 98;
const ACTION_ADD_SOURCE: &'static str = "add_source";

struct App {
  source_list: Rc<RefCell<SourceList>>,
  command_input: CommandInput,
  main_area: Rc<RefCell<MainArea>>,
}

impl App {
  pub fn new() -> App {
    let source_list = Rc::new(RefCell::new(SourceList::new()));
    let command_input = CommandInput::new();
    let main_area = Rc::new(RefCell::new(MainArea::new()));
    App {
      source_list: source_list,
      command_input: command_input,
      main_area: main_area,
    }
  }
  pub fn start(&mut self) {
    self.source_list.borrow().render();
    self.command_input.render();
    self.main_area.borrow_mut().init();
    self.main_area.borrow().render();
    let source_list_clone = self.source_list.clone();
    self.command_input.on_command(move |command: Command| {
      match command.name {
        ACTION_ADD_SOURCE => source_list_clone.borrow_mut().add_source(command.value),
        _ => (),
      };
    });
    let main_area_clone = self.main_area.clone();
    self
      .source_list
      .borrow_mut()
      .on_source_select(move |source: Source| {
        main_area_clone.borrow_mut().load_feed(source);
        main_area_clone.borrow_mut().handle_focus_feed();
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
        FOCUS_FEED_KEY => {
          self.main_area.borrow_mut().handle_focus_feed();
        }
        BACK_TO_FEED_KEY => {
          self.main_area.borrow_mut().handle_back_to_feed();
        }
        _ => (),
      };
    }
  }
}

fn main() {
  let locale_conf = LcCategory::all;
  setlocale(locale_conf, "en_US.UTF-8");
  initscr();
  noecho();
  refresh();
  let mut app = App::new();
  app.start();
}
