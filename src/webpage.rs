pub fn get_document(url: &str) -> String {
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    assert!(response.status().is_success());
    response.text().unwrap()
}
