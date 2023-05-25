mod webpage;
mod word;

use scraper::{Html, Selector};
use std::error::Error;
use word::{Source, Word};

use crate::webpage::get_document;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (html, url) = get_ordnet_page(&config.query);
    let word = build_word(&html, url);

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
    let document = get_document(&url);

    (Html::parse_document(&document), url)
}

pub fn build_word(html: &Html, url: String) -> Word {
    // let article_selector = selector("div.artikel");
    // let article_div = html.select(&article_selector).next().unwrap();
    // println!("{}", article_div.html());

    let ordnet_word = Source {
        value: get_match_value(html),
        group: element_to_string(html, "div.definitionBoxTop span.tekstmedium"),
        bending: element_to_string(html, "#id-boj span.tekstmedium"),
        pronunciation: element_to_string(html, "#id-udt span.tekstmedium"),
        origin: element_to_string(html, "#id-ety span.tekstmedium"),
        url,
    };

    Word::build(ordnet_word)
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
    let element = html.select(&el_selector).next();

    if let Some(el) = element {
        el.text().collect::<String>().trim().to_string()
    } else {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::{SubstantivGroup, WordGroup};
    use std::fs;

    #[test]
    fn can_get_word() {
        let test_html = fs::read_to_string("test/ordnet_fragment.html").unwrap();
        let html = Html::parse_document(&test_html);
        let url = "https://ordnet.dk";
        let parsed_word = build_word(&html, String::from(url));
        let source = Source {
            value: String::from("hygge"),
            group: String::from("substantiv, fælleskøn"),
            bending: String::from("-n"),
            pronunciation: String::from("[ˈhygə]"),
            origin: String::from("dannet af hygge"),
            url: String::from(url),
        };
        let word = Word {
            source,
            value: String::from("en hygge"),
            group: WordGroup::Substantiv(SubstantivGroup::Fælleskon),
        };

        assert_eq!(word.source.value, parsed_word.source.value);
        assert_eq!(word.source.group, parsed_word.source.group);
        assert_eq!(word.source.bending, parsed_word.source.bending);
        assert_eq!(word.source.pronunciation, parsed_word.source.pronunciation);
        assert_eq!(word.source.origin, parsed_word.source.origin);
        assert_eq!(word.source.url, parsed_word.source.url);
        assert_eq!(word.value, parsed_word.value);
        assert_eq!(word.group, parsed_word.group);
    }
}
