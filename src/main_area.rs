use super::feed::Feed;
use super::source_list::Source;
use ncurses::*;

const WIN_FEED: i32 = 0;

pub struct MainArea {
  win: WINDOW,
  active_win: i32,
  feed: Feed,
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
    MainArea {
      win: win,
      active_win: WIN_FEED,
      feed: feed,
    }
  }

  pub fn load_feed(&mut self, source: Source) {
    self.active_win = WIN_FEED;
    self.feed.load_feed(source);
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
    if self.active_win == WIN_FEED {
      self.feed.render();
    }
    wrefresh(self.win);
  }
}