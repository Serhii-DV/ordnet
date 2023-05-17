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
    Ok(())
}

pub fn get_article_content(contents: &str) -> Vec<String> {
    let mut results = Vec::new();

    let fragment = Html::parse_fragment(contents);
    let article_selector = Selector::parse("div.artikel").unwrap();

    let article_div = fragment.select(&article_selector).next().unwrap();
    let article_inner = article_div.inner_html();

    results.push(article_inner);
    results
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

        assert_eq!(vec!["Article content"], get_article_content(contents));
    }
}
