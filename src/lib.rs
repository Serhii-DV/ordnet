use scraper::{ElementRef, Html, Selector};
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
    pub group: String,
    pub is_substantiv: bool,
}

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Running with:\n{}", config.query);

    let html = get_ordnet_page();
    let word = get_ordnet_word(&html);

    output_word(&word);

    Ok(())
}

pub fn get_ordnet_page() -> Html {
    let url = "https://ordnet.dk/ddo/ordbog?query=hygge";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    assert!(response.status().is_success());

    let document = response.text().unwrap();

    Html::parse_document(&document)
}

pub fn get_ordnet_word(html: &Html) -> Word {
    // let article_selector = selector("div.artikel");
    // let article_div = html.select(&article_selector).next().unwrap();
    // println!("{}", article_div.html());

    Word {
        value: get_match_value(html),
        group: get_group_type(html),
        is_substantiv: true,
    }
}

pub fn output_word(word: &Word) {
    println!(
        "Word
    value: {}
    group: {}",
        word.value, word.group
    );
}

fn get_match_value(html: &Html) -> String {
    let text = selector_as_text(html, "div.artikel span.match");
    text.chars().filter(|c| c.is_alphabetic()).collect()
}

fn get_group_type(html: &Html) -> String {
    selector_as_text(html, "div.definitionBoxTop span.tekstmedium")
}

fn create_selector(selector: &'_ str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn el_as_text(element: &ElementRef) -> String {
    element.text().collect::<String>()
}

fn selector_as_text(html: &Html, selector: &'_ str) -> String {
    let el_selector = create_selector(selector);
    let element = html.select(&el_selector).next().unwrap();
    el_as_text(&element)
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
            group: String::from("substantiv, fælleskøn"),
            is_substantiv: true,
        };

        assert_eq!(word.value, get_ordnet_word(&html).value);
    }
}
