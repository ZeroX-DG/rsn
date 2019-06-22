use feed_parser::parser;
use ncurses::*;

const KEY_Q: i32 = 113;
const ENTER: i32 = 10;

#[derive(Clone)]
pub struct Source {
  pub url: String,
  pub title: String,
}

pub struct SourceList {
  win: WINDOW,
  sources: Vec<Source>,
  width: i32,
  selected_index: i32,
  on_source_select: Option<Box<FnMut(Source)>>,
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
    keypad(win, true);
    SourceList {
      win: win,
      sources: Vec::new(),
      width: width,
      selected_index: -1,
      on_source_select: None,
    }
  }

  pub fn add_source(&mut self, source: String) {
    if let Some(feed) = parser::from_url(&source) {
      self.sources.push(Source {
        title: feed.title.unwrap_or(source.clone()),
        url: source.clone(),
      });
      self.render();
    }
  }

  pub fn on_source_select<F: FnMut(Source) + 'static>(&mut self, cb: F) {
    self.on_source_select = Some(Box::new(cb));
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
        }
        KEY_DOWN => {
          if self.selected_index < self.sources.len() as i32 - 1 {
            self.selected_index += 1;
          }
        }
        ENTER => {
          if let Some(cb) = &mut self.on_source_select {
            cb(self.sources[self.selected_index as usize].clone());
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

  pub fn render_sources(&self) {
    if self.sources.len() as i32 == 0 {
      mvwaddstr(self.win, 1, 2, "No source found!");
    } else {
      let mut line = 1;
      for source_data in &self.sources {
        let source = &source_data.title;
        let formatted_source: String = if source.len() as i32 + 3 > self.width {
          format!(
            "{}{}",
            source
              .chars()
              .take((self.width - 5) as usize)
              .collect::<String>(),
            ".."
          )
        } else {
          source.to_string()
        };
        if self.selected_index == line - 1 {
          wattr_on(self.win, A_REVERSE());
          mvwaddstr(self.win, line, 2, &formatted_source);
          wattr_off(self.win, A_REVERSE());
        } else {
          mvwaddstr(self.win, line, 2, &formatted_source);
        }
        line += 1;
      }
    }
    wrefresh(self.win);
  }

  pub fn render(&self) {
    wclear(self.win);
    box_(self.win, 0, 0);
    wattr_on(self.win, A_BOLD());
    mvwaddstr(self.win, 0, 1, "Sources");
    wattr_off(self.win, A_BOLD());
    self.render_sources();
  }
}