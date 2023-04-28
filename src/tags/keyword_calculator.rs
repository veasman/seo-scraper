pub fn calculate_keyword_presence_score(text: &str, keywords: &[String]) -> u32 {
    let mut score = 0;
    let text_lower = text.to_lowercase();

    for keyword in keywords {
        if text_lower.contains(keyword.to_lowercase().as_str()) {
            score += 100 / keywords.len() as u32;
        }
    }

    score
}