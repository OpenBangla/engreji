
pub fn generate_regex(word: &str) -> String {
    let mut output = String::with_capacity(word.len());
    output.push('^');

    let mut chars = word.chars().peekable();
    let mut index = 0;
    while let Some(character) = chars.next() {
        match character.to_ascii_lowercase() {
            'a' => output.push_str("(অ্যা|্যা|া|ে)"),
            'b' => output.push_str("ব?"),
            'c' => {
                if let Some(next) = chars.peek() {
                    // ck -> ক
                    if next.to_ascii_lowercase() == 'k' {
                        output.push_str("ক");
                        // Eat the 'k' character.
                        chars.next();
                        index += 2;
                        continue;
                    } else if next.to_ascii_lowercase() == 'h' {
                        // ch -> চ
                        output.push_str("চ");
                        // Eat the 'h' character.
                        chars.next();
                        index += 2;
                        continue;
                    }
                }
                output.push_str("(ক|স)");
            }
            'd' => output.push_str("(ড)"),
            'e' => output.push_str("(ে|ি|া|ই)?"),
            'f' => output.push_str("ফ"),
            'g' => output.push_str("(গ|জ)?"),
            'h' => output.push_str("(হ)?"),
            'i' => output.push_str("(ই|ি)"),
            'j' => output.push_str("জ?"),
            'k' => output.push_str("(ক)"),
            'l' => output.push_str("ল?"),
            'm' => output.push_str("(ম)?"),
            'n' => {
                // ng -> ং
                if let Some(next) = chars.peek() {
                    if next.to_ascii_lowercase() == 'g' {
                        output.push_str("ং");
                        // Eat the 'g' character.
                        chars.next();
                        index += 2;
                        continue;
                    }
                }
                output.push_str("(ন)");
            }
            'o' => {
                // ou
                if let Some(next) = chars.peek() {
                    if next.to_ascii_lowercase() == 'u' {
                        output.push_str("(াউ|আউ|া|য়া)");
                        // Eat the 'u' character.
                        chars.next();
                        index += 2;
                        continue;
                    }
                }
                output.push_str("(ও|ো|অ)?");
            }
            'p' => output.push_str("প?"),
            'q' => output.push_str("ক"),
            'r' => output.push_str("(র|্র|র্)"),
            's' => output.push_str("(শ|স)?"),
            't' => {
                // tion -> শন
                if let Some(next) = word.get(index..=index+3) {
                    if next.to_ascii_lowercase() == "tion" {
                        output.push_str("শন");
                        // Eat the 'ion' characters.
                        chars.next();
                        chars.next();
                        chars.next();
                        index += 4;
                        continue;
                    }
                }
                output.push_str("(ট)?");
            }
            'u' => output.push_str("(ু|িউ|া)"),
            'v' => output.push_str("ভ?"),
            'w' => output.push_str("(ও|উ)?"),
            'y' => output.push_str("(ি|ই)"),
            _ => ()
        }

        // Add optional Hashanta
        output.push_str("্?");
        index += 1;
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

        regex = Regex::new(&generate_regex("suggestion")).unwrap();
        assert!(regex.is_match("সাজেশন"));

        regex = Regex::new(&generate_regex("start")).unwrap();
        assert!(regex.is_match("স্টার্ট"));

        regex = Regex::new(&generate_regex("chicken")).unwrap();
        assert!(regex.is_match("চিকেন"));

        regex = Regex::new(&generate_regex("attribute")).unwrap();
        assert!(regex.is_match("অ্যাট্রিবিউট"));

        regex = Regex::new(&generate_regex("contribute")).unwrap();
        assert!(regex.is_match("কন্ট্রিবিউট"));

        regex = Regex::new(&generate_regex("package")).unwrap();
        assert!(regex.is_match("প্যাকেজ"));

        regex = Regex::new(&generate_regex("quick")).unwrap();
        assert!(regex.is_match("কুইক"));

        regex = Regex::new(&generate_regex("computation")).unwrap();
        assert!(regex.is_match("কম্পিউটেশন"));

        regex = Regex::new(&generate_regex("feedback")).unwrap();
        assert!(regex.is_match("ফিডব্যাক"));

        regex = Regex::new(&generate_regex("clock")).unwrap();
        assert!(regex.is_match("ক্লক"));

        regex = Regex::new(&generate_regex("out")).unwrap();
        assert!(regex.is_match("আউট"));

        regex = Regex::new(&generate_regex("councillor")).unwrap();
        assert!(regex.is_match("কাউন্সিলর"));

        regex = Regex::new(&generate_regex("double")).unwrap();
        assert!(regex.is_match("ডাবল"));

        regex = Regex::new(&generate_regex("serious")).unwrap();
        assert!(regex.is_match("সিরিয়াস"));
    }
}
