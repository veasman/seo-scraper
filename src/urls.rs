use scraper::{Html, Selector};

pub fn evaluate_url_structure(url: &String, document: &Html) -> u32 {
    let score = 0;

    let link_selector = Selector::parse("a[href]").unwrap();
    let mut internal_links = 0;
    let mut external_links = 0;

    for link_element in document.select(&link_selector) {
        if let Some(link) = link_element.value().attr("href") {
            if link.starts_with("http") && !link.contains(url) {
                external_links += 1;
            } else {
                internal_links += 1;
            }
        }
    }

    println!("Internal links: {}", internal_links);
    println!("External links: {}", external_links);

    score
}