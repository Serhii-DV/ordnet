use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use urlencoding::Encoded;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum WordGroup {
    None,
    Substantiv(SubstantivGroup),
    Verbum,
    Adjektiv,
    Adverbium,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum SubstantivGroup {
    Fælleskon, // n-word
    Intetkøn,  // t-word
}

impl WordGroup {
    pub fn get_prefix(&self) -> &str {
        match *self {
            WordGroup::Substantiv(SubstantivGroup::Fælleskon) => "en",
            WordGroup::Substantiv(SubstantivGroup::Intetkøn) => "et",
            WordGroup::Verbum => "at",
            _ => "",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordSource {
    pub value: String,
    pub group: String,
    pub bending: String,
    pub pronunciation: String,
    pub origin: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub source: WordSource,
    pub value: String,
    pub group: WordGroup,
    pub value_encoded: String,
}

impl Word {
    pub fn build(source: WordSource) -> Self {
        let group = detect_word_group(&source.group);
        let value = get_prefixed_value(&source.value, &group);
        let value_encoded = get_url_encoded(&value);

        Self {
            source,
            value,
            group,
            value_encoded,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn to_custom(&self, template: &str) -> String {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let mut context = Context::new();
        context.insert("word", &self);

        let template_file = template.to_owned() + ".txt";
        tera.render(template_file.as_str(), &context).unwrap()
    }
}

fn detect_word_group(group_text: &str) -> WordGroup {
    let groups = group_text.split(',');

    for part in groups {
        let word_group = match part.trim() {
            "fælleskøn" => WordGroup::Substantiv(SubstantivGroup::Fælleskon),
            "intetkøn" => WordGroup::Substantiv(SubstantivGroup::Intetkøn),
            "verbum" => WordGroup::Verbum,
            "adjektiv" => WordGroup::Adjektiv,
            "adverbium" => WordGroup::Adverbium,
            _ => WordGroup::None,
        };

        if word_group != WordGroup::None {
            return word_group;
        }
    }

    WordGroup::None
}

fn get_prefixed_value(value: &str, group: &WordGroup) -> String {
    let prefix = group.get_prefix();
    let prefix = if prefix.is_empty() {
        prefix.to_owned()
    } else {
        prefix.to_owned() + " "
    };

    prefix + value
}

fn get_url_encoded(word: &str) -> String {
    Encoded(word).to_string()
}

#[cfg(test)]
mod tests {
    use crate::word::{detect_word_group, get_prefixed_value, SubstantivGroup, WordGroup};

    #[test]
    fn can_detect_group() {
        assert_eq!(
            detect_word_group("substantiv, fælleskøn"),
            WordGroup::Substantiv(SubstantivGroup::Fælleskon)
        );
        assert_eq!(
            detect_word_group("substantiv, intetkøn"),
            WordGroup::Substantiv(SubstantivGroup::Intetkøn)
        );
        assert_eq!(detect_word_group("verbum"), WordGroup::Verbum);
        assert_eq!(detect_word_group("adjektiv"), WordGroup::Adjektiv);
        assert_eq!(detect_word_group("adverbium"), WordGroup::Adverbium);
        assert_eq!(detect_word_group("unknown"), WordGroup::None);
    }

    #[test]
    fn can_get_prefixed_value() {
        assert_eq!(
            get_prefixed_value("word", &WordGroup::Substantiv(SubstantivGroup::Fælleskon)),
            "en word"
        );
        assert_eq!(
            get_prefixed_value("word", &WordGroup::Substantiv(SubstantivGroup::Intetkøn)),
            "et word"
        );
    }
}
