use chrono::{DateTime, Utc};

pub fn parse_date(date: &String) -> Option<DateTime<Utc>> {
    if let Ok(date) = format!("{}:00Z", date).parse::<DateTime<Utc>>() {
        return Some(date);
    };
    return None;
}
