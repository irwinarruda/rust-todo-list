pub fn are_similar(str1: &String, str2: &String) -> bool {
    return str1
        .to_lowercase()
        .replace(" ", "")
        .contains(&str2.to_lowercase().replace(" ", ""));
}
