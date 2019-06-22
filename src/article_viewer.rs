use feed_parser::Entry;
use ncurses::*;

pub struct ArticleViewer {
  win: WINDOW,
  article: Option<Entry>,
}

impl ArticleViewer {
  pub fn new(height: i32, width: i32, y: i32, x: i32) -> ArticleViewer {
    let win = newwin(height, width, y, x);
    keypad(win, true);
    ArticleViewer {
      win: win,
      article: None,
    }
  }

  pub fn set_article(&mut self, article: Entry) {
    self.article = Some(article);
  }

  pub fn render(&self) {
    mvwaddstr(self.win, 1, 1, "YO");
    if let Some(article) = &self.article {
      if let Some(title) = &article.title {
        wattr_on(self.win, A_BOLD());
        mvwaddstr(self.win, 0, 1, title);
        wattr_off(self.win, A_BOLD());
      }
      if let Some(content) = &article.content {
        mvwaddstr(self.win, 1, 1, content);
      }
    }
    wrefresh(self.win);
  }
}