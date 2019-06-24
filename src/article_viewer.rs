use super::util;
use feed_parser::Entry;
use html2text::from_read;
use ncurses::*;
use readability::extractor;
use webbrowser;

const KEY_Q: i32 = 113;
const KEY_O: i32 = 111;

pub struct ArticleViewer {
  win: WINDOW,
  title: String,
  content: String,
  url: String,
  width: i32,
  height: i32,
  scroll_top: i32,
  scrollable: bool,
  lines: i32,
}

impl ArticleViewer {
  pub fn new(height: i32, width: i32, y: i32, x: i32) -> ArticleViewer {
    let win = newwin(height, width, y, x);
    keypad(win, true);
    ArticleViewer {
      win: win,
      title: String::new(),
      content: String::new(),
      url: String::new(),
      width: width,
      height: height,
      scroll_top: 0,
      scrollable: false,
      lines: 0,
    }
  }

  pub fn set_article(&mut self, article: Entry) {
    self.scroll_top = 0;
    if let Some(title) = &article.title {
      self.title = title.to_string();
    }
    if let Some(link) = &article.alternate.first() {
      let scraped_result = extractor::scrape(&link.href);
      let article_content = match scraped_result {
        Ok(scraped_data) => from_read(scraped_data.content.as_bytes(), self.width as usize),
        Err(_) => String::from("Failed to fetch article"),
      };
      let article_content_clone = article_content.clone();
      self.content = article_content;
      self.url = link.href.clone();
      self.lines = article_content_clone.lines().count() as i32;
      self.scrollable = self.lines > self.height;
    }
    self.handle_focus();
  }

  pub fn handle_focus(&mut self) {
    self.render();
    loop {
      let ch = wgetch(self.win);
      match ch {
        KEY_DOWN => {
          // allow scroll to half screen
          if self.scrollable && self.scroll_top + self.lines > self.height / 2 {
            self.scroll_top -= 1;
          }
        }
        KEY_UP => {
          if self.scrollable && self.scroll_top < 0 {
            self.scroll_top += 1;
          }
        }
        KEY_Q => {
          break;
        }
        KEY_O => {
          webbrowser::open(&self.url).unwrap();
        }
        _ => {}
      }
      self.render();
    }
  }

  pub fn render(&self) {
    wclear(self.win);
    wattr_on(self.win, A_BOLD());
    mvwaddstr(self.win, self.scroll_top, 0, &format!("{}", &self.title));
    wattr_off(self.win, A_BOLD());
    let mut y = 3;
    let parts = util::parse_effects(String::from(self.content.clone()));
    let mut pos = 0;
    for part in parts {
      if part.0 == "\n" {
        y += 1;
        pos = -1;
      } else if part.1 == "normal" {
        mvwaddstr(self.win, y + self.scroll_top, pos, &part.0);
      } else {
        let effect = if part.1 == "bold" {
          A_BOLD()
        } else if part.1 == "code" {
          A_REVERSE()
        } else {
          A_ITALIC()
        };
        wattr_on(self.win, effect);
        mvwaddstr(self.win, y + self.scroll_top, pos, &part.0);
        wattr_off(self.win, effect);
      }
      pos += part.0.chars().count() as i32
    }
    wrefresh(self.win);
  }
}