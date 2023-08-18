use scraper::{Html, Selector};

pub fn get_document(url: &str) -> String {
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    assert!(response.status().is_success());
    response.text().unwrap()
}

pub fn create_selector(selector: &'_ str) -> Selector {
    Selector::parse(selector).unwrap()
}

pub fn element_to_string(html: &Html, selector: &'_ str) -> String {
    let el_selector = create_selector(selector);
    let element = html.select(&el_selector).next();

    if let Some(el) = element {
        el.text().collect::<String>().trim().to_string()
    } else {
        String::from("")
    }
}
