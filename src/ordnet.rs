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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::word::Source;
    use scraper::Html;

    use super::build_source;

    #[test]
    fn can_build_source() {
        let test_html = fs::read_to_string("test/ordnet_fragment.html").unwrap();
        let html = Html::parse_document(&test_html);
        let url = "https://ordnet.dk";
        let word_source = build_source(&html, String::from(url));

        let assert_source = Source {
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
