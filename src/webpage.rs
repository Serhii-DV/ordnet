use scraper::{ElementRef, Html, Selector};

pub fn get_document(url: &str) -> Html {
    let html = get_html(url);
    Html::parse_document(&html)
}

pub fn get_html(url: &str) -> String {
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    assert!(response.status().is_success());
    response.text().unwrap()
}

pub fn create_selector(selector: &'_ str) -> Selector {
    Selector::parse(selector).unwrap()
}

pub fn element_by_selector_to_string(document: &Html, selector: &'_ str) -> String {
    let el_selector = create_selector(selector);
    let element = document.select(&el_selector).next();

    if let Some(el) = element {
        element_to_string(el)
    } else {
        String::from("")
    }
}

pub fn sub_element_by_selector_to_string(element: ElementRef, selector: &'_ str) -> String {
    let el_selector = create_selector(selector);
    let sub_element = element.select(&el_selector).next();

    if let Some(el) = sub_element {
        element_to_string(el)
    } else {
        String::from("")
    }
}

pub fn element_to_string(element: ElementRef) -> String {
    element.text().collect::<String>().trim().to_string()
}

pub fn extract_elements_as_text_vec(element: ElementRef, selector: &'_ str) -> Vec<String> {
    // Define the CSS selector
    let selector = Selector::parse(selector).unwrap();

    // Extract the content of selected elements and store it in a Vec<String>
    let elements_as_text_vec: Vec<String> = element
        .select(&selector)
        .map(|element| {
            let text = element.text().collect::<String>();
            text
        })
        .collect();

    elements_as_text_vec
}
