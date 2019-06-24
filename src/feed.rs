use super::source_list::Source;
use feed_parser::{parser, Entry};
use ncurses::*;

const ENTER: i32 = 10;
const KEY_Q: i32 = 113;

pub struct Feed {
  win: WINDOW,
  width: i32,
  height: i32,
  scroll_top: i32,
  feed: Vec<Entry>,
  selected_index: i32,
  on_entry_select: Option<Box<FnMut(Entry)>>,
}

impl Feed {
  pub fn new(height: i32, width: i32, y: i32, x: i32) -> Feed {
    let win = newwin(height, width, y, x);
    keypad(win, true);
    Feed {
      win: win,
      width: width,
      height: height,
      scroll_top: 0,
      feed: Vec::new(),
      selected_index: -1,
      on_entry_select: None,
    }
  }

  pub fn load_feed(&mut self, source: Source) {
    if let Some(feed) = parser::from_url(&source.url) {
      self.feed = feed.entries;
      self.selected_index = 0;
      self.render();
    }
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

          if self.selected_index * 3 + self.scroll_top < 0 {
            self.scroll_top += 4;
          }
        }
        KEY_DOWN => {
          if self.selected_index < self.feed.len() as i32 - 1 {
            self.selected_index += 1;
          }

          if self.selected_index * 3 + self.scroll_top > self.height {
            self.scroll_top -= 4;
          }
        }
        ENTER => {
          if let Some(cb) = &mut self.on_entry_select {
            if self.feed.len() as i32 > self.selected_index && self.selected_index != -1 {
              cb(self.feed[self.selected_index as usize].clone());
            }
          };
          break;
        }
        KEY_Q => {
          break;
        }
        _ => {}
      };
      self.render();
    }
  }

  pub fn on_entry_select<F: FnMut(Entry) + 'static>(&mut self, cb: F) {
    self.on_entry_select = Some(Box::new(cb));
  }

  pub fn render(&self) {
    wclear(self.win);
    if self.feed.len() as i32 == 0 {
      mvwaddstr(self.win, 0, 1, "Nothing to see here!");
    } else {
      let mut line = self.scroll_top;
      for entry in &self.feed {
        self.render_entry(line, entry);
        line += 3;
      }
    }
    wrefresh(self.win);
  }

  pub fn render_entry(&self, y: i32, entry: &Entry) {
    // Render title
    if let Some(title) = &entry.title {
      let formatted_title: String = if title.len() as i32 + 2 > self.width {
        format!(
          "{}{}",
          title
            .chars()
            .take((self.width - 4) as usize)
            .collect::<String>(),
          ".."
        )
      } else {
        title.to_string()
      };

      if self.selected_index * 3 + self.scroll_top == y {
        wattr_on(self.win, A_REVERSE());
        mvwaddstr(self.win, y, 1, &formatted_title);
        wattr_off(self.win, A_REVERSE());
      } else {
        wattr_on(self.win, A_BOLD());
        mvwaddstr(self.win, y, 1, &formatted_title);
        wattr_off(self.win, A_BOLD());
      }
    }

    // Render meta
    // date
    mvwaddstr(
      self.win,
      y + 1,
      1,
      &entry.published.format("%Y-%m-%d").to_string(),
    );
  }
}