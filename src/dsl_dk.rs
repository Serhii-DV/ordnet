use scraper::Html;

use crate::{
    webpage::{element_to_string, get_document},
    word::{Word, WordSource},
};

pub fn build_word(query: &str) -> Word {
    let url = get_query_url(query);
    let document = get_document(&url);
    let word_source = build_source(&document, &url);

    Word::build(word_source)
}

fn get_query_url(query: &str) -> String {
    "https://ws.dsl.dk/ddo/query?q={QUERY}".replace("{QUERY}", query)
}

fn build_source(document: &Html, url: &str) -> WordSource {
    WordSource {
        value: element_to_string(document, ".ar .head .k"),
        group: element_to_string(document, ".ar .pos"),
        bending: element_to_string(document, "#id-boj span.tekstmedium"),
        pronunciation: element_to_string(document, ".ar .phon"),
        origin: element_to_string(document, ".ar .etym"),
        url: String::from(url),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::word::WordSource;
    use scraper::Html;

    use super::build_source;

    #[test]
    fn can_build_source() {
        let test_html = fs::read_to_string("test/dsl/1/desuden.html").unwrap();
        let document = Html::parse_document(&test_html);
        let url = "https://ws.dsl.dk/ddo";
        let word_source = build_source(&document, url);

        let assert_source = WordSource {
            value: String::from("desuden"),
            group: String::from("adverbium"),
            bending: String::from(""),
            pronunciation: String::from("[desˈuːðən]"),
            origin: String::from("første led des genitiv singularis af det i partitiv betydning, egentlig '(for)uden af det'"),
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
