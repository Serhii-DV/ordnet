use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub value_text: String,
    pub value: String,
    pub group_text: String,
    pub group: WordGroup,
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

    pub fn to_custom(&self, template: &str) -> String {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let mut context = Context::new();
        context.insert("word", self);

        tera.render(template, &context).unwrap()
    }
}

pub fn detect_word_group(group_text: &str) -> WordGroup {
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

pub fn generate_word_value(raw_value: &str, word_group: &WordGroup) -> String {
    let prefix = match *word_group {
        WordGroup::Substantiv(SubstantivGroup::Fælleskon) => "en ",
        WordGroup::Substantiv(SubstantivGroup::Intetkøn) => "et ",
        _ => "",
    };

    prefix.to_owned() + raw_value
}

#[cfg(test)]
mod tests {
    use crate::word::{detect_word_group, SubstantivGroup, WordGroup};

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
}
