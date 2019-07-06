use super::util;
use darkside::region::*;
use darkside::text::*;
use darkside::Border;
use feed_parser::Entry;
use html2text::*;
use readability::*;

pub struct ArticleViewer {
  title: String,
  content: String,
  url: String,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  scroll_top: i32,
  scrollable: bool,
  lines: i32,
}

pub fn new_article_viewer(x: i32, y: i32, width: i32, height: i32) -> ArticleViewer {
  ArticleViewer {
    title: String::new(),
    content: String::new(),
    url: String::new(),
    x: x,
    y: y,
    width: width,
    height: height,
    scroll_top: 0,
    scrollable: false,
    lines: 0,
  }
}

pub fn set_article(viewer: ArticleViewer, article: &Entry) -> ArticleViewer {
  let mut update_viewer = viewer;
  update_viewer.scroll_top = 0;
  if let Some(title) = &article.title {
    update_viewer.title = title.to_string();
  }
  if let Some(link) = &article.alternate.first() {
    let scraped_result = extractor::scrape(&link.href);
    let article_content = match scraped_result {
      Ok(scraped_data) => from_read(
        scraped_data.content.as_bytes(),
        update_viewer.width as usize,
      ),
      Err(_) => String::from("Failed to fetch article"),
    };
    let article_content_clone = article_content.clone();
    update_viewer.content = article_content;
    update_viewer.url = link.href.clone();
    update_viewer.lines = article_content_clone.lines().count() as i32;
    update_viewer.scrollable = update_viewer.lines > update_viewer.height;
  }
  update_viewer
}

pub fn viewer_scroll_down(viewer: ArticleViewer) -> ArticleViewer {
  let mut update_viewer = viewer;
  if update_viewer.scrollable
    && update_viewer.scroll_top + update_viewer.lines > update_viewer.height / 2
  {
    update_viewer.scroll_top -= 1;
  }
  update_viewer
}

pub fn viewer_scroll_up(viewer: ArticleViewer) -> ArticleViewer {
  let mut update_viewer = viewer;
  if update_viewer.scrollable && update_viewer.scroll_top < 0 {
    update_viewer.scroll_top += 1;
  }
  update_viewer
}

pub fn open_article(viewer: &ArticleViewer) {
  webbrowser::open(&viewer.url).unwrap();
}

pub fn render_article_viewer(viewer: &ArticleViewer) {
  let container_region = new_region(
    viewer.x,
    viewer.y,
    viewer.width,
    viewer.height,
    None,
    Border::None,
  );
  let mut title = new_text(&viewer.title, 0, viewer.scroll_top);
  title = set_text_effects(title, vec![TextEffect::Bold]);
  title = set_text_region(title, &container_region);
  render_text(&title);

  let mut y = 3;
  let parts = util::parse_effects(viewer.content.clone());
  let mut pos = 0;
  for part in parts {
    if part.0 == "\n" {
      y += 1;
      pos = -1;
    } else if part.1 == "normal" {
      let mut line = new_text(&part.0, pos, y + viewer.scroll_top);
      line = set_text_region(line, &container_region);
      render_text(&line);
    } else {
      let effect = if part.1 == "bold" {
        TextEffect::Bold
      } else if part.1 == "code" {
        TextEffect::Highlighted
      } else {
        TextEffect::Italic
      };
      let mut line = new_text(&part.0, pos, y + viewer.scroll_top);
      line = set_text_effects(line, vec![effect]);
      line = set_text_region(line, &container_region);
      render_text(&line);
    }
    pos += part.0.chars().count() as i32
  }
}
