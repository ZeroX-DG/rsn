use feed_parser::Entry;
use html2text::from_read;
use ncurses::*;
use readability::extractor;

pub struct ArticleViewer {
  win: WINDOW,
  article: Option<Entry>,
  width: i32,
}

impl ArticleViewer {
  pub fn new(height: i32, width: i32, y: i32, x: i32) -> ArticleViewer {
    let win = newwin(height, width, y, x);
    keypad(win, true);
    ArticleViewer {
      win: win,
      article: None,
      width: width,
    }
  }

  pub fn set_article(&mut self, article: Entry) {
    self.article = Some(article);
  }

  pub fn render(&self) {
    if let Some(article) = &self.article {
      if let Some(title) = &article.title {
        wattr_on(self.win, A_BOLD());
        mvwaddstr(self.win, 0, 0, &format!("{}\n", title));
        wattr_off(self.win, A_BOLD());
      }
      if let Some(link) = &article.alternate.first() {
        let scraped_result = extractor::scrape(&link.href);
        match scraped_result {
          Ok(scraped_data) => waddstr(
            self.win,
            &from_read(scraped_data.content.as_bytes(), self.width as usize),
          ),
          Err(_) => waddstr(self.win, "Failed to fetch article"),
        };
      }
    }
    wrefresh(self.win);
  }
}