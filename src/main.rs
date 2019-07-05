extern crate darkside;
extern crate dirs;
extern crate readability;
extern crate serde;
extern crate serde_json;
extern crate webbrowser;

mod article_viewer;
mod user_data;
mod util;

use feed_parser::{parser, Entry};

use serde::{Deserialize, Serialize};

use article_viewer::*;
use darkside::input::*;
use darkside::list::*;
use darkside::region::*;
use darkside::*;
use std::char;
use user_data::UserData;

enum Parts {
  MainApp,
  CommandInput,
  SourceList,
  MainArea,
}

#[derive(Deserialize, Serialize)]
pub struct Source {
  pub url: String,
  pub title: String,
}

fn main() {
  new_app();
  let mut active_part: Parts = Parts::MainApp;
  let mut active_feed: Option<Vec<Entry>> = None;
  let mut show_feed = false;
  let term_size = get_term_size();
  let term_width = term_size.0;
  let term_height = term_size.1;
  let user_data = UserData::load();
  let mut command_input = new_input(0, term_height - 1, term_width, "", false);

  let source_list_width = term_width * 30 / 100;
  let source_list_region = new_region(
    0,
    0,
    source_list_width,
    term_height - 1,
    Some("Sources"),
    Border::All,
  );
  let sources_display = user_data
    .sources
    .iter()
    .map(|source: &Source| source.title.clone())
    .collect::<Vec<String>>();
  let mut source_list = new_list(
    2,
    1,
    source_list_width - 4,
    term_height - 3,
    sources_display,
  );

  let main_area_width = term_width - source_list_width;
  let main_area_region = new_region(
    source_list_width,
    0,
    main_area_width,
    term_height - 1,
    None,
    Border::All,
  );

  let mut feed_list = new_list(
    source_list_width + 2,
    1,
    main_area_width - 4,
    term_height - 3,
    vec![],
  );

  feed_list = set_list_fill_width(feed_list, true);
  feed_list = set_list_item_spacing(feed_list, 1);
  feed_list = set_list_item_height(feed_list, 2);
  feed_list = set_list_text_overflow(feed_list, TextOverflow::Ellipsis);

  let mut article_viewer = new_article_viewer(
    source_list_width + 2,
    1,
    main_area_width - 4,
    term_height - 3,
  );
  loop {
    render_region(&main_area_region);
    render_region(&source_list_region);
    render_list(&source_list);
    match active_part {
      Parts::CommandInput => render_input(&command_input),
      _ => (),
    };
    if show_feed {
      if let Some(feed) = &active_feed {
        let feed_display = feed
          .iter()
          .map(|entry: &Entry| {
            let title = match &entry.title {
              Some(t) => t,
              None => "",
            };
            format!("{}\n{}", title, entry.published)
          })
          .collect::<Vec<String>>();
        feed_list = set_list_items(feed_list, feed_display);
        render_list(&feed_list);
      }
    } else {
      render_article_viewer(&article_viewer);
    }
    let ch = wait_for_key();
    if ch == translate_key('i') {
      command_input = set_input_prompt(command_input, "Add source");
      active_part = Parts::CommandInput;
      continue;
    } else if ch == translate_key('l') {
      active_part = Parts::SourceList;
      continue;
    } else if ch == translate_key('f') {
      active_part = Parts::MainArea;
      continue;
    }
    match active_part {
      Parts::CommandInput => {
        if ch == KEY_RETURN {
          active_part = Parts::MainApp;
        } else {
          if let Some(char_input) = char::from_u32(ch as u32) {
            command_input = add_input_char(command_input, char_input);
          }
        }
      }
      Parts::SourceList => {
        if ch == KEY_DOWN {
          source_list = move_next_list_item(source_list);
        } else if ch == KEY_UP {
          source_list = move_prev_list_item(source_list);
        } else if ch == KEY_RETURN {
          let index = get_list_selected_index(&source_list);
          let source = &user_data.sources[index as usize];
          if let Some(feed) = parser::from_url(&source.url) {
            active_feed = Some(feed.entries);
            show_feed = true;
            active_part = Parts::MainArea;
          }
        }
      }
      Parts::MainArea => {
        if show_feed {
          if ch == KEY_DOWN {
            feed_list = move_next_list_item(feed_list);
          } else if ch == KEY_UP {
            feed_list = move_prev_list_item(feed_list);
          } else if ch == KEY_RETURN {
            let index = get_list_selected_index(&feed_list);
            if let Some(feed) = &active_feed {
              let selected_article = &feed[index as usize];
              article_viewer = set_article(article_viewer, selected_article);
              show_feed = false;
            }
          }
        } else {
          if ch == KEY_DOWN {
            article_viewer = viewer_scroll_down(article_viewer);
          } else if ch == KEY_UP {
            article_viewer = viewer_scroll_up(article_viewer);
          } else if ch == translate_key('o') {
            open_article(&article_viewer);
          } else if ch == translate_key('b') {
            show_feed = true;
          }
        }
      }
      _ => (),
    }
  }
}