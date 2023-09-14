mod dsl_dk;
mod ordnet;
mod webpage;
mod word;

use std::error::Error;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Query value
    query: String,

    /// Format type
    #[arg(short, long, default_value = "default")]
    format: Option<String>,

    /// Source type
    #[arg(short, long, value_enum, default_value_t=Source::Dsl)]
    source: Source,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Source {
    /// Source ordnet.dk
    Ordnet,
    /// Source dsl.dk (faster)
    Dsl,
}

#[derive(Debug)]
pub enum Format {
    Json,
    JsonPretty,
    Custom(String),
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub format: Format,
    pub source: Source,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let args = Args::parse();
        let query = args.query;
        let format = match args.format {
            Some(ref s) if s == "json" => Format::Json,
            Some(ref s) if s == "json-pretty" => Format::JsonPretty,
            Some(custom_value) => Format::Custom(custom_value.to_string()),
            None => Format::Custom(String::from("default")),
        };
        let source = args.source;

        Ok(Config {
            query,
            format,
            source,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let word = match config.source {
        Source::Ordnet => ordnet::build_word(&config.query),
        Source::Dsl => dsl_dk::build_word(&config.query),
    };

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
    use crate::word::{SubstantivGroup, Word, WordGroup, WordSource};
    use std::fs;

    #[test]
    fn can_build_word() {
        let test_html = fs::read_to_string("test/ordnet_fragment.html").unwrap();
        let html = Html::parse_document(&test_html);
        let url = "https://ordnet.dk";
        let word_source_ordnet = ordnet::build_source(&html, url);
        let parsed_word = Word::build(word_source_ordnet);
        let source = WordSource {
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
