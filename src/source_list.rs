use feed_parser::parser;
use ncurses::*;
use serde::{Deserialize, Serialize};

const KEY_Q: i32 = 113;
const KEY_D: i32 = 100;
const ENTER: i32 = 10;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
  pub url: String,
  pub title: String,
}

pub struct SourceList {
  win: WINDOW,
  inner_win: WINDOW,
  sources: Vec<Source>,
  width: i32,
  height: i32,
  selected_index: i32,
  scroll_top: i32,
  on_source_select: Option<Box<FnMut(Source)>>,
  on_source_added: Option<Box<FnMut(Source)>>,
  on_source_removed: Option<Box<FnMut(Vec<Source>)>>,
}

impl SourceList {
  pub fn new(sources: Vec<Source>) -> SourceList {
    let mut screen_w = 0;
    let mut screen_h = 0;
    getmaxyx(stdscr(), &mut screen_h, &mut screen_w);
    // 30% of terminal width
    let width = 30 * screen_w / 100;
    let height = screen_h - 1;
    let win = newwin(height, width, 0, 0);
    let inner_win = newwin(height - 2, width - 2, 1, 1);
    keypad(win, true);
    SourceList {
      win: win,
      inner_win: inner_win,
      sources: sources,
      width: width,
      height: height,
      selected_index: -1,
      scroll_top: 0,
      on_source_select: None,
      on_source_added: None,
      on_source_removed: None,
    }
  }

  pub fn add_source(&mut self, source: String) {
    if let Some(feed) = parser::from_url(&source) {
      let new_source = Source {
        title: feed.title.unwrap_or(source.clone()),
        url: source.clone(),
      };
      self.sources.push(new_source.clone());
      if let Some(cb) = &mut self.on_source_added {
        cb(new_source.clone())
      }
      self.render();
    }
  }

  pub fn on_source_select<F: FnMut(Source) + 'static>(&mut self, cb: F) {
    self.on_source_select = Some(Box::new(cb));
  }

  pub fn on_source_added<F: FnMut(Source) + 'static>(&mut self, cb: F) {
    self.on_source_added = Some(Box::new(cb));
  }

  pub fn on_source_removed<F: FnMut(Vec<Source>) + 'static>(&mut self, cb: F) {
    self.on_source_removed = Some(Box::new(cb));
  }

  pub fn handle_focus(&mut self) {
    if self.selected_index == -1 {
      self.selected_index = 0;
      self.render();
    }
    loop {
      let ch = wgetch(self.win);
      match ch {
        KEY_UP => {
          if self.selected_index > 0 {
            self.selected_index -= 1;
          }

          if self.selected_index + self.scroll_top < 0 {
            self.scroll_top += 1;
          }
        }
        KEY_DOWN => {
          if self.selected_index < self.sources.len() as i32 - 1 {
            self.selected_index += 1;
          }

          if self.selected_index + self.scroll_top > self.height - 3 {
            self.scroll_top -= 1;
          }
        }
        ENTER => {
          if let Some(cb) = &mut self.on_source_select {
            if self.sources.len() as i32 > self.selected_index && self.selected_index != -1 {
              cb(self.sources[self.selected_index as usize].clone());
            }
          };
          break;
        }
        KEY_D => {
          self.sources.remove(self.selected_index as usize);
          self.selected_index = -1;
          if let Some(cb) = &mut self.on_source_removed {
            cb(self.sources.clone());
          }
        }
        KEY_Q => {
          break;
        }
        _ => {}
      };
      self.render();
    }
  }

  pub fn render_sources(&self) {
    if self.sources.len() as i32 == 0 {
      mvwaddstr(self.inner_win, 0, 0, "No source found!");
    } else {
      let mut line = self.scroll_top;
      for source_data in &self.sources {
        let source = &source_data.title;
        let max_width = self.width - 6;
        let formatted_source: String = if source.len() as i32 > max_width {
          format!(
            "{}{}",
            source.chars().take(max_width as usize).collect::<String>(),
            ".."
          )
        } else {
          source.to_string()
        };
        if self.selected_index + self.scroll_top == line {
          wattr_on(self.inner_win, A_REVERSE());
          mvwaddstr(self.inner_win, line, 1, &formatted_source);
          wattr_off(self.inner_win, A_REVERSE());
        } else {
          mvwaddstr(self.inner_win, line, 1, &formatted_source);
        }
        line += 1;
      }
    }
    wrefresh(self.win);
    wrefresh(self.inner_win);
  }

  pub fn render(&self) {
    wclear(self.win);
    wclear(self.inner_win);
    box_(self.win, 0, 0);
    wattr_on(self.win, A_BOLD());
    mvwaddstr(self.win, 0, 1, "Sources");
    wattr_off(self.win, A_BOLD());
    self.render_sources();
  }
}