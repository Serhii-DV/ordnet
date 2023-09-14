use scraper::Html;

use crate::webpage::element_to_string;
use crate::word::Source;

pub fn get_query_url(query: &str) -> String {
    "https://ordnet.dk/ddo/ordbog?query={QUERY}".replace("{QUERY}", query)
}

pub fn build_source(html: &Html, url: String) -> Source {
    Source {
        value: get_match_value(html),
        group: element_to_string(html, "div.definitionBoxTop span.tekstmedium"),
        bending: element_to_string(html, "#id-boj span.tekstmedium"),
        pronunciation: element_to_string(html, "#id-udt span.tekstmedium"),
        origin: element_to_string(html, "#id-ety span.tekstmedium"),
        url,
    }
}

fn get_match_value(html: &Html) -> String {
    let text = element_to_string(html, "div.artikel span.match");
    text.chars().filter(|c| c.is_alphabetic()).collect()
}
