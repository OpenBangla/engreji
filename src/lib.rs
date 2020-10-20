
pub fn generate_regex(word: &str) -> String {
    let mut output = String::with_capacity(word.len());
    output.push('^');

    let mut chars = word.chars().peekable();
    while let Some(character) = chars.next() {
        match character.to_ascii_lowercase() {
            'a' => output.push_str("(অ্যা|্যা|া)"),
            'c' => output.push_str("(ক)"),
            'd' => output.push_str("(ড)"),
            'e' => output.push_str("(ে|ই)"),
            'g' => output.push_str("(গ|জ)"),
            'h' => output.push_str("(হ)?"),
            'i' => output.push_str("(ই|ি)"),
            'l' => output.push_str("(ল)"),
            'm' => output.push_str("(ম)?"),
            'n' => {
                // ng => ং
                if let Some(next) = chars.peek() {
                    if next.to_ascii_lowercase() == 'g' {
                        output.push_str("ং");
                        // Eat the 'g' character.
                        chars.next();
                        continue;
                    }
                }
                output.push_str("(ন)");
            }
            'o' => output.push_str("(ও|ো)?"),
            'p' => output.push_str("প"),
            'r' => output.push_str("(র|্র)"),
            's' => output.push_str("(শ|স)"),
            _ => ()
        }

        // Add optional Hashanta
        output.push_str("্?");
    }
    output.push('$');

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

        regex = Regex::new(&generate_regex("english")).unwrap();
        assert!(regex.is_match("ইংলিশ"));

        regex = Regex::new(&generate_regex("programming")).unwrap();
        assert!(regex.is_match("প্রোগ্রামিং"));
    }
}
