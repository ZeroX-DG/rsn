use ncurses::*;
use std::char;

const ENTER: i32 = 10; // ENTER KEY, some how ncurses-rs ENTER KEY doesn't match

pub struct Command {
  pub name: &'static str,
  pub value: String,
}

pub struct CommandInput {
  win: WINDOW,
  input: String,
  prompt: &'static str,
  command_name: &'static str,
  on_command: Option<Box<FnMut(Command)>>,
}

impl CommandInput {
  pub fn new() -> CommandInput {
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let win = newwin(1, max_x, max_y - 1, 0);
    keypad(win, true);
    CommandInput {
      win: win,
      input: String::new(),
      prompt: "",
      command_name: "",
      on_command: None,
    }
  }

  pub fn set_command_name(&mut self, name: &'static str) {
    self.command_name = name;
  }

  pub fn prompt(&mut self, prompt: &'static str) {
    self.prompt = prompt;
    self.render();
    loop {
      let ch_raw = wgetch(self.win);
      let ch: char = char::from_u32(ch_raw as u32).unwrap();
      match ch_raw {
        KEY_BACKSPACE => {
          let input_len = self.input.len();
          if input_len > 0 {
            self.input.remove(input_len - 1);
          }
        }
        ENTER => {
          if let Some(cb) = &mut self.on_command {
            cb(Command {
              name: &self.command_name,
              value: self.input.clone(),
            });
          };
          self.input.clear();
          self.prompt = "";
          self.render();
          break;
        }
        _ => {
          self.input = format!("{}{}", self.input, ch);
        }
      };
      self.render();
    }
  }

  pub fn on_command<F: FnMut(Command) + 'static>(&mut self, cb: F) {
    self.on_command = Some(Box::new(cb));
  }

  pub fn render(&self) {
    wclear(self.win);
    wattr_on(self.win, A_BOLD());
    waddstr(self.win, self.prompt);
    wattr_off(self.win, A_BOLD());
    waddstr(self.win, &self.input);
    wrefresh(self.win);
  }
}