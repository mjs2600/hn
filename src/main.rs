extern crate colored;
extern crate reqwest;
extern crate scraper;

use colored::*;
use scraper::{ElementRef, Html, Selector};

fn main() {
  let parsed_body = Html::parse_document(&get_body());

  for element in parsed_body.select(&selector()) {
    parse_story(element)
  }
}

fn get_body() -> String {
  reqwest::get("https://news.ycombinator.com/")
    .expect("Can't reach Hacker News")
    .text()
    .unwrap()
}

fn selector() -> Selector {
  Selector::parse("a.storylink").unwrap()
}

fn parse_link(element: ElementRef) -> String {
  let mut link = String::from(element.value().attr("href").unwrap());
  if link.starts_with("item?") {
    link = format!("https://news.ycombinator.com/{}", link);
  }
  link
}

fn parse_story(element: ElementRef) {
  let title_element = element.first_child().unwrap().value();
  if let scraper::Node::Text(title_text) = title_element {
    let link = parse_link(element);
    let title = String::from(&title_text.text);
    println!("{}", title.cyan());
    println!("\t{}", link);
  }
}
