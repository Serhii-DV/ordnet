use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub enum Format {
    Json,
    JsonPretty,
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
                _ => Format::Json,
            }
        } else {
            Format::Json
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
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (html, url) = get_ordnet_page(&config.query);
    let word = get_ordnet_word(&html, &url);

    println!(
        "{}",
        match config.format {
            Format::Json => word.to_json(),
            Format::JsonPretty => word.to_json_pretty(),
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
        group: selector_as_text(html, "div.definitionBoxTop span.tekstmedium"),
        is_substantiv: true,
        bending: selector_as_text(html, "#id-boj span.tekstmedium"),
        pronunciation: selector_as_text(html, "#id-udt span.tekstmedium"),
        origin: selector_as_text(html, "#id-ety span.tekstmedium"),
        url: String::from(url),
    }
}

fn get_match_value(html: &Html) -> String {
    let text = selector_as_text(html, "div.artikel span.match");
    text.chars().filter(|c| c.is_alphabetic()).collect()
}

fn create_selector(selector: &'_ str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn el_as_text(element: &ElementRef) -> String {
    element.text().collect::<String>().trim().to_string()
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
    <div class=\"definitionBox\" id=\"id-boj\">
        <span class=\"stempel\">Bøjning</span>
        <span class=\"tekstmedium allow-glossing\">-n</span>
    </div>
    <div class=\"definitionBox details\" id=\"id-udt\">
        <span class=\"stempel\">Udtale</span>
        <span class=\"tekstmedium allow-glossing\">
            <span class=\"lydskrift\">
                <span class=\"diskret\">[</span>ˈhygə<span class=\"diskret\">]</span>&nbsp;
                <audio id=\"11022047_1\" src=\"https://static.ordnet.dk/mp3/11022/11022047_1.mp3\">
                    <div class=\"hiddenStructure\">
                        <a href=\"https://static.ordnet.dk/mp3/11022/11022047_1.mp3\" id=\"11022047_1_fallback\">&nbsp;</a>
                    </div>
                </audio>
                <img src=\"speaker.gif\" style=\"cursor: pointer;\" onclick=\"playSound('11022047_1'); _gaq.push(['_trackPageview', '/static/lyd/11022047_1']);\">
            </span>
        </span>
        <span class=\"tipIkon\">
        <a href=\"../ddo/artiklernes-opbygning/udtale?set_language=da\" title=\"hjælp til læsning af lydskriften\">
            <img src=\"tip_ikon_mini.gif\" alt=\"tip\" width=\"18\" height=\"18\">
        </a>
        </span>
    </div>
    <div class=\"definitionBox details\" id=\"id-ety\">
        <span class=\"stempel\">Oprindelse</span>
        <span class=\"tekstmedium allow-glossing\">dannet af <a href=\"?entry_id=11022048&amp;query=hygge\">hygge</a></span>
    </div>
</div>";
        let html = Html::parse_document(html);
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
        assert_eq!(word.url, parsed_word.url);
    }
}
