mod dsl_dk;
mod ordnet;
mod webpage;
mod word;

use std::error::Error;

pub enum Format {
    Json,
    JsonPretty,
    Custom(String),
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
            match args[2].as_str() {
                "json" => Format::Json,
                "json-pretty" => Format::JsonPretty,
                custom_value => Format::Custom(custom_value.to_string()),
            }
        } else {
            Format::Custom(String::from("default"))
        };

        Ok(Config { query, format })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let word = ordnet::build_word(&config.query);
    let _word = dsl_dk::build_word(&config.query);

    println!(
        "{}",
        match config.format {
            Format::Json => word.to_json(),
            Format::JsonPretty => word.to_json_pretty(),
            Format::Custom(value) => word.to_custom(value.as_str()),
        }
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    use super::*;
    use crate::word::{Source, SubstantivGroup, Word, WordGroup};
    use std::fs;

    #[test]
    fn can_build_word() {
        let test_html = fs::read_to_string("test/ordnet_fragment.html").unwrap();
        let html = Html::parse_document(&test_html);
        let url = "https://ordnet.dk";
        let word_source_ordnet = ordnet::build_source(&html, url);
        let parsed_word = Word::build(word_source_ordnet);
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
            value_encoded: String::from(""),
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
