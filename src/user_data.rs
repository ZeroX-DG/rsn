use super::source_list::Source;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct UserData {
  pub sources: Vec<Source>,
}

impl UserData {
  pub fn empty() -> UserData {
    UserData {
      sources: Vec::new(),
    }
  }
  pub fn load() -> UserData {
    match dirs::home_dir() {
      Some(path) => {
        let data_path = path.to_str().unwrap();
        let data_dir = format!("{}/{}/", data_path, ".rsn");
        let data_file = format!("{}/{}", data_dir, "user_data.json");
        if !Path::new(&data_dir).exists() {
          fs::create_dir(&data_dir).unwrap();
          fs::write(
            &data_file,
            serde_json::to_string(&UserData::empty()).unwrap(),
          )
          .unwrap();
        }
        let utf8data = fs::read(&data_file).unwrap();
        let data = String::from_utf8_lossy(&utf8data);
        let user_data: UserData = serde_json::from_str(&data).unwrap();
        user_data
      }
      None => panic!("Can't get home path"),
    }
  }

  pub fn add_source(&mut self, source: Source) {
    self.sources.push(source);
  }

  pub fn set_sources(&mut self, sources: Vec<Source>) {
    self.sources = sources;
  }

  pub fn save(&self) {
    match dirs::home_dir() {
      Some(path) => {
        let data_path = path.to_str().unwrap();
        let data_dir = format!("{}/{}/", data_path, ".rsn");
        let data_file = format!("{}/{}", data_dir, "user_data.json");
        if !Path::new(&data_dir).exists() {
          fs::create_dir(&data_dir).unwrap();
        }
        fs::write(&data_file, serde_json::to_string(&self).unwrap()).unwrap();
      }
      None => panic!("Can't get home path"),
    }
  }
}