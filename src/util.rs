pub fn parse_effects(text: String) -> Vec<(String, &'static str)> {
  let mut result: Vec<(String, &'static str)> = vec![];
  let mut chars = text.chars();
  let mut effect = "normal";
  let mut ch = chars.next();
  let mut sentence: String = String::from("");

  while ch != None {
    if ch.unwrap() == '*' {
      let next_ch = chars.next();
      if next_ch != None && next_ch.unwrap() == '*' {
        if effect != "bold" {
          effect = "bold";
        } else {
          effect = "normal";
        }
      } else if next_ch != None && next_ch.unwrap() != ' ' {
        if effect != "italic" {
          effect = "italic";
        } else {
          effect = "normal";
        }
        sentence.push(next_ch.unwrap());
      }
      result.push((sentence.clone(), effect));
      sentence.clear();
    } else if ch.unwrap() == '`' {
      if effect != "code" {
        effect = "code";
      } else {
        effect = "normal";
      }
      result.push((sentence.clone(), effect));
      sentence.clear();
    } else {
      sentence.push(ch.unwrap());
    }
    result.push((sentence.clone(), effect));
    sentence.clear();
    ch = chars.next();
  }

  result
}