use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tera::{Context, Tera};

pub enum Format {
    Json,
    JsonPretty,
    Custom,
}

pub struct Config {
    pub query: String,
    pub format: Format,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let format = if args.get(2).is_some() {
            match args[2].clone().as_str() {
                "json" => Format::Json,
                "json-pretty" => Format::JsonPretty,
                _ => Format::Custom,
            }
        } else {
            Format::Custom
        };

        Ok(Config { query, format })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub value: String,
    pub group: String,
    pub is_substantiv: bool,
    pub bending: String,
    pub pronunciation: String,
    pub origin: String,
    pub url: String,
}

impl Word {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn to_custom(&self, template: &str) -> String {
        let tera = match Tera::new("template/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let mut context = Context::new();
        context.insert("word", self);

        tera.render(template, &context).unwrap()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (html, url) = get_ordnet_page(&config.query);
    let word = get_ordnet_word(&html, &url);

    println!(
        "{}",
        match config.format {
            Format::Json => word.to_json(),
            Format::JsonPretty => word.to_json_pretty(),
            Format::Custom => word.to_custom("ankiweb.html"),
        }
    );

    Ok(())
}

pub fn get_ordnet_page(query: &str) -> (Html, String) {
    let url = "https://ordnet.dk/ddo/ordbog?query={QUERY}".replace("{QUERY}", query);
    let response = reqwest::blocking::get(&url).expect("Could not load url.");
    assert!(response.status().is_success());

    let document = response.text().unwrap();

    (Html::parse_document(&document), url)
}

pub fn get_ordnet_word(html: &Html, url: &str) -> Word {
    // let article_selector = selector("div.artikel");
    // let article_div = html.select(&article_selector).next().unwrap();
    // println!("{}", article_div.html());

    Word {
        value: get_match_value(html),
        group: element_to_string(html, "div.definitionBoxTop span.tekstmedium"),
        is_substantiv: true,
        bending: element_to_string(html, "#id-boj span.tekstmedium"),
        pronunciation: element_to_string(html, "#id-udt span.tekstmedium"),
        origin: element_to_string(html, "#id-ety span.tekstmedium"),
        url: String::from(url),
    }
}

fn get_match_value(html: &Html) -> String {
    let text = element_to_string(html, "div.artikel span.match");
    text.chars().filter(|c| c.is_alphabetic()).collect()
}

fn create_selector(selector: &'_ str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn element_to_string(html: &Html, selector: &'_ str) -> String {
    let el_selector = create_selector(selector);
    let element = html.select(&el_selector).next().unwrap();
    element.text().collect::<String>().trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn can_get_word() {
        let test_html = fs::read_to_string("test/ordnet_fragment.html").unwrap();
        let html = Html::parse_document(&test_html);
        let url = String::from("https://ordnet.dk");
        let parsed_word = get_ordnet_word(&html, &url);
        let word = Word {
            value: String::from("hygge"),
            group: String::from("substantiv, fælleskøn"),
            is_substantiv: true,
            bending: String::from("-n"),
            pronunciation: String::from("[ˈhygə]"),
            origin: String::from("dannet af hygge"),
            url,
        };

        assert_eq!(word.value, parsed_word.value);
        assert_eq!(word.group, parsed_word.group);
        assert_eq!(word.is_substantiv, parsed_word.is_substantiv);
        assert_eq!(word.bending, parsed_word.bending);
        assert_eq!(word.pronunciation, parsed_word.pronunciation);
        assert_eq!(word.origin, parsed_word.origin);
        assert_eq!(word.url, parsed_word.url);
    }
}
