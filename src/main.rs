use std::io::{self, Write};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html};

mod tags;
mod urls;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (url, keywords): (String, Vec<String>) = if args.len() > 2 {
        (args[1].clone(), args.iter().skip(2).map(|s| s.clone()).collect::<Vec<_>>())
    } else {
        let mut input_url = String::new();
        let mut input_keywords = String::new();

        print!("Enter URL: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_url).expect("Failed to read URL");

        print!("Enter keywords (space-separated): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_keywords).expect("Failed to read keywords");

        let input_keywords_vec = input_keywords
            .trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        (input_url.trim().to_string(), input_keywords_vec)
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"),
    );

    let response = reqwest::blocking::Client::new()
        .get(url.clone())
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let document = Html::parse_document(&response);

    let title_score = tags::evaluate_titles(&document, &keywords);
    let meta_descriptions_score = tags::elavuate_meta_descriptions(&document, &keywords);
    let header_tags_score = tags::evaluate_headers(&document);
    let url_structure_score = urls::evaluate_url_structure(&url.clone(), &document);

    println!("Title and Score: {}", title_score);
    println!("Meta Descriptions Score: {}", meta_descriptions_score);
    println!("Header Tags Score: {}", header_tags_score);
    println!("URL Structure Score: {}", url_structure_score);
}
