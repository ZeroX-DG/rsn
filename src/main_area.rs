use super::article_viewer::ArticleViewer;
use super::feed::Feed;
use super::source_list::Source;

use std::cell::RefCell;
use std::rc::Rc;

use feed_parser::Entry;
use ncurses::*;

const WIN_ARTICLE: i32 = 1;
const WIN_FEED: i32 = 0;

#[derive(Clone)]
pub struct MainArea {
  win: Rc<RefCell<WINDOW>>,
  active_win: Rc<RefCell<i32>>,
  feed: Rc<RefCell<Feed>>,
  article_viewer: Rc<RefCell<ArticleViewer>>,
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
    let feed = Feed::new(height - 1, width - 2, 0, x + 1);
    let article_viewer = ArticleViewer::new(height - 1, width - 2, 0, x + 1);

    MainArea {
      win: Rc::new(RefCell::new(win)),
      active_win: Rc::new(RefCell::new(WIN_FEED)),
      feed: Rc::new(RefCell::new(feed)),
      article_viewer: Rc::new(RefCell::new(article_viewer)),
    }
  }

  pub fn init(&mut self) {
    let self_clone = self.clone();
    self.feed.borrow_mut().on_entry_select(move |entry: Entry| {
      *self_clone.active_win.borrow_mut() = WIN_ARTICLE;
      self_clone.article_viewer.borrow_mut().set_article(entry);
      self_clone.render();
    });
  }

  pub fn load_feed(&mut self, source: Source) {
    *self.active_win.borrow_mut() = WIN_FEED;
    self.feed.borrow_mut().load_feed(source);
  }

  pub fn handle_focus_feed(&mut self) {
    *self.active_win.borrow_mut() = WIN_FEED;
    self.feed.borrow_mut().handle_focus();
  }

  pub fn render(&self) {
    wclear(*self.win.borrow());
    box_(*self.win.borrow(), 0, 0);
    wattr_on(*self.win.borrow(), A_BOLD());
    let title = if *self.active_win.borrow() == WIN_FEED {
      "Feed"
    } else {
      "Article viewer"
    };
    mvwaddstr(*self.win.borrow(), 0, 1, title);
    wattr_off(*self.win.borrow(), A_BOLD());
    if *self.active_win.borrow() == WIN_FEED {
      self.feed.borrow().render();
    } else {
      self.article_viewer.borrow().render();
    }
    wrefresh(*self.win.borrow());
  }
}