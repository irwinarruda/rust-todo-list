pub fn format_bool(b: bool) -> String {
    return if b {
        "Yes".to_string()
    } else {
        "No".to_string()
    };
}
