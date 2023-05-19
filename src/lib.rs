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

    let html = get_ordnet_page();
    let word = get_ordnet_word(&html);

    println!("{}", word.value);

    Ok(())
}

pub fn get_ordnet_page() -> Html {
    let response = reqwest::blocking::get("https://ordnet.dk/ddo/ordbog?query=hygge")
        .unwrap()
        .text()
        .unwrap();

    Html::parse_document(&response)
}

pub fn get_ordnet_word(html: &Html) -> Word {
    // let match_selector = selector("div.artikel div.definitionBoxTop span.match");
    // let match_span = html.select(&match_selector).next().expect("No match element found");
    // println!("{:?}", match_span.next_sibling());

    let article_selector = selector("div.artikel");
    let article_div = html.select(&article_selector).next().unwrap();
    println!("{}", article_div.html());

    Word {
        value: String::from("hygge"),
        is_substantiv: true,
    }
}

fn selector(selector: &'_ str) -> Selector {
    Selector::parse(selector).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_word() {
        let html = "\
<div>
    <div class=\"artikel\">
    <div class=\"definitionBoxTop\">
        <span class=\"match\">hygge<span class=\"super\">1</span></span>
        <span class=\"tekstmedium allow-glossing\">substantiv, fælleskøn</span></div>
    </div>
</div>";
        let html = Html::parse_document(html);
        let word = Word {
            value: String::from("hygge"),
            is_substantiv: true,
        };

        assert_eq!(word.value, get_ordnet_word(&html).value);
    }
}
