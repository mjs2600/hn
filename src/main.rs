use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use std::fmt;
use std::result::Result;

#[derive(Debug)]
struct Site {
    name: String,
    stories: Vec<Story>,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for story in &self.stories {
            writeln!(f, "{}", story)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Story {
    title: String,
    link: Option<String>,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.link {
            Some(link) => write!(f, "\t{}\n\t\t({})", self.title, link),
            None => write!(f, "\t{}", self.title),
        }
    }
}

fn main() {
    match hacker_news() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }

    match datatau() {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching DataTau data."),
    }
}

fn hacker_news() -> Result<Site, Error> {
    let body = get_html("https://news.ycombinator.com/")?;
    let stories = body
        .select(&selector())
        .map(|element| Story {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site {
        name: "Hacker News".to_string(),
        stories,
    };

    Ok(site)
}

fn datatau() -> Result<Site, Error> {
    let body = get_html("https://datatau.net/")?;
    let stories = body
        .select(&selector())
        .map(|element| Story {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site {
        name: "DataTau".to_string(),
        stories,
    };

    Ok(site)
}

fn get_html(uri: &str) -> Result<Html, Error> {
    Ok(Html::parse_document(&reqwest::blocking::get(uri)?.text()?))
}

fn selector() -> Selector {
    Selector::parse("a.storylink").unwrap()
}

fn parse_link(element: ElementRef) -> Option<String> {
    let mut link: Option<String> = None;
    if let Some(link_str) = element.value().attr("href") {
        let mut link_str = link_str.to_owned();
        if link_str.starts_with("item?") {
            link_str = format!("https://news.ycombinator.com/{}", link_str);
        }
        link = Some(link_str);
    }

    link
}

fn parse_title(element: ElementRef) -> String {
    element.inner_html()
}
