use scraper::{Html, Selector};

mod keyword_calculator;

pub fn evaluate_titles(document: &Html, keywords: &[String]) -> u32 {
    let mut score = 0;

    let title_selector = Selector::parse("head > title").unwrap();
    if let Some(title_element) = document.select(&title_selector).next() {
        let title_text = title_element.text().collect::<String>();

        let ideal_title_length = 55;

        let title_length_diff = (title_text.len() as i32 - ideal_title_length).abs();

        let title_length_score = 100 - title_length_diff;

        let title_keyword_score = keyword_calculator::calculate_keyword_presence_score(&title_text, keywords);

        score += (0.7 * title_length_score as f64 + 0.3 * title_keyword_score as f64) as u32;
    }

    score
}

pub fn elavuate_meta_descriptions(document: &Html, keywords: &[String]) -> u32 {
    let mut score = 0;

    let meta_description_selector = Selector::parse(r#"meta[name="description"]"#).unwrap();
    if let Some(meta_description_element) = document.select(&meta_description_selector).next() {
        let meta_description = meta_description_element.value().attr("content").unwrap_or("");

        println!("meta description: {}", meta_description);

        let ideal_meta_description_length = 155;

        let meta_description_length_diff = (meta_description.len() as i32 - ideal_meta_description_length).abs();
        let meta_description_length_score = 100 - meta_description_length_diff;

        let meta_description_keyword_score = keyword_calculator::calculate_keyword_presence_score(meta_description, keywords);

        score += (0.7 * meta_description_length_score as f64 + 0.3 * meta_description_keyword_score as f64) as u32;
    }

    score
}

pub fn evaluate_headers(document: &Html) -> u32 {
    let score = 0;

    for i in 1..=6 {
        let heading_selector = Selector::parse(&format!("h{}", i)).unwrap();
        let heading_count = document.select(&heading_selector).count();
        println!("Number of h{} tags: {}", i, heading_count);
    }

    score
}