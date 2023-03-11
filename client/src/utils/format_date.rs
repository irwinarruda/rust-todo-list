use chrono::{DateTime, TimeZone};

pub fn format_date<T: TimeZone>(date: &DateTime<T>) -> String
where
    T::Offset: std::fmt::Display,
{
    return date.format("%d/%m/%Y %H:%M").to_string();
}
