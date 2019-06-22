use ncurses::*;
use feed_parser::{parser, Entry};
use super::source_list::Source;

pub struct Feed {
  win: WINDOW,
  width: i32,
  feed: Vec<Entry>
}

impl Feed {
  pub fn new(height: i32, width: i32, y: i32, x: i32) -> Feed {
    let win = newwin(height, width, y, x);
    Feed {
      win: win,
      width: width,
      feed: Vec::new()
    }
  }

  pub fn load_feed(&mut self, source: Source) {
    if let Some(feed) = parser::from_url(&source.url) {
      self.feed = feed.entries;
      self.render();
    }
  }

  pub fn render(&self) {
    if self.feed.len() as i32 == 0 {
      mvwaddstr(self.win, 1, 1, "Nothing to see here!");
    } else {
      let mut line = 1;
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
}