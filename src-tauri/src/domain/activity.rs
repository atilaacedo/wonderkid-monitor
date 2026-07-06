#[derive(Debug)]
pub enum Activity {
    Programming(String),
    Studying(String),
    Gaming(String),
    Distraction,
    Unknown,
}

impl Activity {
    pub fn classify(window_title: &str) -> Self {
        let title_lower = window_title.to_lowercase();
        if title_lower.contains("code") || title_lower.contains("rust") || title_lower.contains("nvim") {
            Activity::Programming(window_title.to_string())
        } else if title_lower.contains("steam") || title_lower.contains("discord") {
            Activity::Gaming(window_title.to_string())
        } else {
            Activity::Distraction
        }
    }
}
