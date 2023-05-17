use scraper::{Html, Selector};
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running with:\n{}", config.query);

    let response = reqwest::blocking::get("https://ordnet.dk/ddo/ordbog?query=hygge")
        .unwrap()
        .text()
        .unwrap();

    let article_content = get_article_content(&response);

    println!("{}", article_content);

    Ok(())
}

pub fn get_article_content(contents: &str) -> String {
    let fragment = Html::parse_fragment(contents);
    let article_selector = Selector::parse("div.artikel").unwrap();

    let article_div = fragment.select(&article_selector).next().unwrap();
    article_div.inner_html()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_article_content() {
        let contents = "\
<div>
    <div class=\"artikel\">Article content</div>
</div>";

        assert_eq!("Article content", get_article_content(contents));
    }
}
