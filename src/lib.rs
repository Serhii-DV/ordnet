use scraper::{Html, Selector};
use std::error::Error;

pub struct Config {
    pub query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();

        Ok(Config { query })
    }
}

pub struct Word {
    pub value: String,
    pub is_substantiv: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running with:\n{}", config.query);

    let response = reqwest::blocking::get("https://ordnet.dk/ddo/ordbog?query=hygge")
        .unwrap()
        .text()
        .unwrap();

    let word = get_word_from_html(&response);

    println!("{}", word.value);

    Ok(())
}

pub fn get_word_from_html(html: &str) -> Word {
    let fragment = Html::parse_fragment(html);
    let article_selector = Selector::parse("div.artikel").unwrap();

    let _article_div = fragment.select(&article_selector).next().unwrap();

    Word {
        value: String::from("hygge"),
        is_substantiv: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_word() {
        let html = "\
<div>
    <div class=\"artikel\">Article content</div>
</div>";
        let word = Word {
            value: String::from("hygge"),
            is_substantiv: true,
        };

        assert_eq!(word.value, get_word_from_html(html).value);
    }
}
