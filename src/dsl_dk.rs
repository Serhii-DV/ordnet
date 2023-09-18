use scraper::{ElementRef, Html};

use crate::{
    webpage::{
        create_selector, extract_elements_as_text_vec, get_document,
        sub_element_by_selector_to_string,
    },
    word::{Word, WordSource},
};

pub fn build_words(query: &str) -> Vec<Word> {
    let url = get_query_url(query);
    let document = get_document(&url);
    let word_sources = build_word_sources(&document, query);
    let mut words: Vec<Word> = Vec::new();

    for word_source in word_sources {
        let word = Word::build(word_source);
        words.push(word);
    }

    words
}

fn get_query_url(query: &str) -> String {
    "https://ws.dsl.dk/ddo/query?q={QUERY}".replace("{QUERY}", query)
}

fn build_source_from_element(element: ElementRef, url: &str) -> WordSource {
    WordSource {
        value: sub_element_by_selector_to_string(element, ".ar .head .k"),
        group: sub_element_by_selector_to_string(element, ".ar .pos"),
        bending: sub_element_by_selector_to_string(element, "#id-boj span.tekstmedium"),
        pronunciation: sub_element_by_selector_to_string(element, ".ar .phon"),
        origin: sub_element_by_selector_to_string(element, ".ar .etym"),
        synonyms: extract_elements_as_text_vec(element, ".synonyms .k a"),
        url: String::from(url),
    }
}

fn build_word_sources(document: &Html, url: &str) -> Vec<WordSource> {
    let mut word_sources: Vec<WordSource> = Vec::new();
    let selector = &create_selector(".ar");
    let elements = document.select(selector);

    for element in elements {
        let word_source = build_source_from_element(element, url);
        word_sources.push(word_source);
    }

    word_sources
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::word::WordSource;
    use scraper::Html;

    use super::build_word_sources;

    #[test]
    fn can_build_source_for_single_variant() {
        let url = "https://ws.dsl.dk/ddo";
        let test_html = fs::read_to_string("test/dsl/1/desuden.html").unwrap();
        let document = Html::parse_document(&test_html);
        let word_sources = build_word_sources(&document, url);
        let expected_word_sources: Vec<WordSource> = vec![
            WordSource {
                value: String::from("desuden"),
                group: String::from("adverbium"),
                bending: String::from(""),
                pronunciation: String::from("[desˈuːðən]"),
                origin: String::from("første led des genitiv singularis af det i partitiv betydning, egentlig '(for)uden af det'"),
                synonyms: vec![
                    String::from("derudover"),
                    String::from("desforuden"),
                    String::from("ovenikøbet"),
                    String::from("i øvrigt"),
                    String::from("for resten"),
                ],
                url: String::from(url),
            }
        ];

        word_sources
            .iter()
            .enumerate()
            .for_each(|(index, word_source)| {
                if let Some(expected_word_source) = expected_word_sources.get(index) {
                    assert_word_source_eq(word_source, expected_word_source);
                }
            });
    }

    #[test]
    fn can_build_sources_for_multiple_variants() {
        let url = "https://ws.dsl.dk/ddo";
        let test_html = fs::read_to_string("test/dsl/3/vokser.html").unwrap();
        let document = Html::parse_document(&test_html);
        let word_sources = build_word_sources(&document, url);
        let expected_word_sources: Vec<WordSource> = vec![
            WordSource {
                value: String::from("voks"),
                group: String::from("substantiv, fælleskøn eller intetkøn"),
                bending: String::from(""),
                pronunciation: String::from("[ˈvʌgs]"),
                origin: String::from(""),
                synonyms: Vec::new(),
                url: String::from(url),
            },
            WordSource {
                value: String::from("vokse1"),
                group: String::from("verbum"),
                bending: String::from(""),
                pronunciation: String::from("[ˈvʌgsə]"),
                origin: String::from(""),
                synonyms: Vec::new(),
                url: String::from(url),
            },
            WordSource {
                value: String::from("vokse2"),
                group: String::from("verbum"),
                bending: String::from(""),
                pronunciation: String::from("[ˈvʌgsə]"),
                origin: String::from(""),
                synonyms: Vec::new(),
                url: String::from(url),
            },
        ];

        assert_eq!(word_sources.len(), 3);

        word_sources
            .iter()
            .enumerate()
            .for_each(|(index, word_source)| {
                if let Some(expected_word_source) = expected_word_sources.get(index) {
                    assert_word_source_eq(word_source, expected_word_source);
                }
            });
    }

    fn assert_word_source_eq(word_source: &WordSource, expected_word_source: &WordSource) {
        assert_eq!(word_source.value, expected_word_source.value);
        assert_eq!(word_source.group, expected_word_source.group);
        assert_eq!(word_source.bending, expected_word_source.bending);
        assert_eq!(
            word_source.pronunciation,
            expected_word_source.pronunciation
        );
        // assert_eq!(word_source.origin, expected_word_source.origin);
        assert_eq!(word_source.url, expected_word_source.url);
    }
}
