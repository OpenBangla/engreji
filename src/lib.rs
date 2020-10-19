
pub fn generate_regex(word: &str) -> String {
    let mut output = String::with_capacity(word.len());

    for character in word.chars() {
        match character.to_ascii_lowercase() {
            'a' => output.push_str("(অ্যা|্যা|া)"),
            'c' => output.push_str("(ক)"),
            'd' => output.push_str("(ড)"),
            'e' => output.push_str("(ে)"),
            'g' => output.push_str("(গ|জ)"),
            'l' => output.push_str("(ল)"),
            'h' => output.push_str("(হ)"),
            'm' => output.push_str("(ম)?"),
            'n' => output.push_str("(ন)"),
            'o' => output.push_str("(ও|ো)?"),
            _ => ()
        }

        // Add optional Hashanta
        output.push_str("্?");
    }

    output
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::generate_regex;
    #[test]
    fn test() {
        let mut regex;

        regex = Regex::new(&generate_regex("halogen")).unwrap();
        assert!(regex.is_match("হ্যালোজেন"));

        regex = Regex::new(&generate_regex("command")).unwrap();
        assert!(regex.is_match("কমান্ড"));
    }
}
