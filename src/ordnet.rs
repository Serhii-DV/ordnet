use scraper::Html;

use crate::webpage::{element_to_string, get_document};
use crate::word::{Word, WordSource};

pub fn get_query_url(query: &str) -> String {
    "https://ordnet.dk/ddo/ordbog?query={QUERY}".replace("{QUERY}", query)
}

pub fn build_word(query: &str) -> Word {
    let url = get_query_url(query);
    let html = get_document(&url);
    let word_source = build_source(&html, &url);

    Word::build(word_source)
}

pub fn build_source(document: &Html, url: &str) -> WordSource {
    WordSource {
        value: get_match_value(document),
        group: element_to_string(document, "div.definitionBoxTop span.tekstmedium"),
        bending: element_to_string(document, "#id-boj span.tekstmedium"),
        pronunciation: element_to_string(document, "#id-udt span.tekstmedium"),
        origin: element_to_string(document, "#id-ety span.tekstmedium"),
        url: String::from(url),
    }
}

fn get_match_value(document: &Html) -> String {
    let text = element_to_string(document, "div.artikel span.match");
    text.chars().filter(|c| c.is_alphabetic()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::word::WordSource;
    use scraper::Html;

    use super::build_source;

    #[test]
    fn can_build_source() {
        let test_html = fs::read_to_string("test/ordnet_fragment.html").unwrap();
        let document = Html::parse_document(&test_html);
        let url = "https://ordnet.dk";
        let word_source = build_source(&document, url);

        let assert_source = WordSource {
            value: String::from("hygge"),
            group: String::from("substantiv, fælleskøn"),
            bending: String::from("-n"),
            pronunciation: String::from("[ˈhygə]"),
            origin: String::from("dannet af hygge"),
            url: String::from(url),
        };

        assert_eq!(assert_source.value, word_source.value);
        assert_eq!(assert_source.group, word_source.group);
        assert_eq!(assert_source.bending, word_source.bending);
        assert_eq!(assert_source.pronunciation, word_source.pronunciation);
        assert_eq!(assert_source.origin, word_source.origin);
        assert_eq!(assert_source.url, word_source.url);
    }
}
