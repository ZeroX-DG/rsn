use super::source_list::Source;
use chrono::prelude::*;
use feed_parser::{parser, Entry};
use ncurses::*;

const WIN_FEED: i32 = 0;

pub struct MainArea {
  win: WINDOW,
  width: i32,
  active_win: i32,
  selected_index: i32,
  feed: Vec<Entry>,
}

impl MainArea {
  pub fn new() -> MainArea {
    let mut screen_w = 0;
    let mut screen_h = 0;
    getmaxyx(stdscr(), &mut screen_h, &mut screen_w);
    // 30% of terminal width is the source list
    let x = 30 * screen_w / 100;
    // 70% of terminal width
    let width = 70 * screen_w / 100;
    let height = screen_h - 1;
    let win = newwin(height, width, 0, x);
    keypad(win, true);
    MainArea {
      win: win,
      active_win: WIN_FEED,
      width: width,
      selected_index: -1,
      feed: Vec::new(),
    }
  }

  pub fn load_feed(&mut self, source: Source) {
    self.active_win = WIN_FEED;
    if let Some(feed) = parser::from_url(&source.url) {
      self.feed = feed.entries;
      self.render();
    }
  }

  pub fn render_feed(&self) {
    if self.feed.len() as i32 == 0 {
      mvwaddstr(self.win, 1, 1, "Nothing to see here!");
    } else {
      let mut line = 1;
      for entry in &self.feed {
        self.render_entry(line, entry);
        line += 3;
      }
    }
  }

  pub fn render_entry(&self, y: i32, entry: &Entry) {
    // Render title
    if let Some(title) = &entry.title {
      let formatted_title: String = if title.len() as i32 > self.width {
        format!(
          "{}{}",
          title
            .chars()
            .take((self.width - 6) as usize)
            .collect::<String>(),
          ".."
        )
      } else {
        title.to_string()
      };

      wattr_on(self.win, A_BOLD());
      mvwaddstr(self.win, y, 1, &formatted_title);
      wattr_off(self.win, A_BOLD());
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

  pub fn render(&self) {
    wclear(self.win);
    box_(self.win, 0, 0);
    wattr_on(self.win, A_BOLD());
    let title = if self.active_win == WIN_FEED {
      "Feed"
    } else {
      ""
    };
    mvwaddstr(self.win, 0, 1, title);
    wattr_off(self.win, A_BOLD());
    self.render_feed();
    wrefresh(self.win);
  }
}