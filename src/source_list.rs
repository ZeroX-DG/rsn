use ncurses::*;

#[derive(Clone)]
pub struct SourceList {
  win: WINDOW,
  sources: Vec<String>,
}

impl SourceList {
  pub fn new() -> SourceList {
    let mut screen_w = 0;
    let mut screen_h = 0;
    getmaxyx(stdscr(), &mut screen_h, &mut screen_w);
    // 30% of terminal width
    let width = 30 * screen_w / 100;
    let height = screen_h - 1;
    let win = newwin(height, width, 0, 0);
    SourceList {
      win: win,
      sources: Vec::new(),
    }
  }

  pub fn add_source(&mut self, source: String) {
    self.sources.push(source);
    self.render_sources();
  }

  pub fn render_sources(&self) {
    if self.sources.len() as i32 == 0 {
      mvwaddstr(self.win, 1, 1, "No source found!");
    } else {
      let mut line = 1;
      for source in &self.sources {
        mvwaddstr(self.win, line, 1, &source);
        line += 1;
      }
    }
    wrefresh(self.win);
  }

  pub fn render(&self) {
    box_(self.win, 0, 0);
    self.render_sources();
  }
}