use scraper::Html;

use crate::webpage::{element_to_string, get_document};
use crate::word::{Source, Word};

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
