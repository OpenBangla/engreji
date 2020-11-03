
pub fn generate_regex(word: &str) -> String {
    let mut output = String::with_capacity(word.len());
    output.push('^');

    let mut chars = word.char_indices().peekable();
    while let Some((index, character)) = chars.next() {
        match character.to_ascii_lowercase() {
            'a' => output.push_str("(আ|অ্যা|্যা|া|ে|অ|য়ে|য়া|এ)?"),
            'b' => output.push_str("ব?"),
            'c' => {
                if let Some((_, next)) = chars.peek() {
                    let next = next.to_ascii_lowercase();
                    // ck -> ক
                    if next == 'k' {
                        output.push_str("ক");
                        // Eat the 'k' character.
                        chars.next();
                        continue;
                    } else if next == 'h' {
                        // ch -> চ
                        output.push_str("চ|ক");
                        // Eat the 'h' character.
                        chars.next();
                        continue;
                    }
                }
                output.push_str("(ক|স|শ)?");
            }
            'd' => output.push_str("(ড)?"),
            'e' => output.push_str("(ে|ি|া|ই|এ|য়)?"),
            'f' => output.push_str("ফ?"),
            'g' => output.push_str("(গ|জ)?"),
            'h' => output.push_str("(হ)?"),
            'i' => output.push_str("(ই|ি|া|াই|ে|আই|য়াই)?"),
            'j' => output.push_str("জ?"),
            'k' => output.push_str("(ক)"),
            'l' => {
                // le
                if let Some((_, next)) = chars.peek() {
                    if next.to_ascii_lowercase() == 'e' {
                        output.push_str("(ল|লে|লি|েল|িল)");
                        // Eat the 'e' character.
                        chars.next();
                        continue;
                    }
                }
                output.push_str("ল?");
            }
            'm' => output.push_str("(ম)?"),
            'n' => {
                if let Some((_, next)) = chars.peek() {
                    let next = next.to_ascii_lowercase();
                    // ng -> ং, ঙ্গ, ঞ্জ
                    if next == 'g' {
                        output.push_str("(ং|ঙ্গ|ঞ্জ)");
                        // Eat the 'g' character.
                        chars.next();
                        continue;
                    } else if next == 'j' {
                        output.push_str("ঞ্জ");
                        // Eat the 'j' character.
                        chars.next();
                        continue;
                    } else if next == 'k' {
                        // nk -> ংক, ঙ্ক
                        output.push_str("(ংক|ঙ্ক)");
                        // Eat the 'k' character.
                        chars.next();
                        continue;
                    }
                }
                output.push_str("ন?");
            }
            'o' => {
                if let Some((_, next)) = chars.peek() {
                    let next = next.to_ascii_lowercase();
                    // ou
                    if next == 'u' {
                        output.push_str("(াউ|আউ|া|ো|য়া|ু)");
                        // Eat the 'u' character.
                        chars.next();
                        continue;
                    } else if next == 'w' {
                        // ow
                        output.push_str("(াউ)");
                        // Eat the 'w' character.
                        chars.next();
                        continue;
                    }
                }
                output.push_str("(ও|ো|অ|য়|য়ো|া|ু)?");
            }
            'p' => {
                // ph -> ফ
                if let Some((_, next)) = chars.peek() {
                    if next.to_ascii_lowercase() == 'h' {
                        output.push_str("ফ");
                        // Eat the 'h' character.
                        chars.next();
                        continue;
                    }
                }
                output.push_str("প?");
            }
            'q' => output.push_str("ক"),
            'r' => output.push_str("(র|্র|র্|র\u{200d}|ার)?"),
            's' => {
                // sion -> শন, সন
                if let Some(next) = word.get(index..=index+3) {
                    if next.to_ascii_lowercase() == "sion" {
                        output.push_str("(শন|সন)");
                        // Eat the 'ion' characters.
                        chars.next();
                        chars.next();
                        chars.next();
                        continue;
                    }
                }
                output.push_str("(শ|স|জ)?");
            }
            't' => {
                // th -> থ
                if let Some((_, next)) = chars.peek() {
                    if next.to_ascii_lowercase() == 'h' {
                        output.push_str("(থ|দ)");
                        // Eat the 'h' character.
                        chars.next();
                        continue;
                    }
                }
                // tion -> শন
                if let Some(next) = word.get(index..=index+3) {
                    if next.to_ascii_lowercase() == "tion" {
                        output.push_str("শন");
                        // Eat the 'ion' characters.
                        chars.next();
                        chars.next();
                        chars.next();
                        continue;
                    }
                }
                output.push_str("(ট|চ)?");
            }
            'u' => output.push_str("(ু|িউ|ইউ|া|আ|য়া)?"),
            'v' => output.push_str("ভ?"),
            'w' => output.push_str("(ও|উ)?"),
            'x' => output.push_str("(ক্স|জ)?"),
            'y' => output.push_str("(ি|ই|াই|ে)"),
            'z' => output.push_str("জ?"),
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
