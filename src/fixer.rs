use regex::Regex;

pub fn fix_msg(content: &str) -> Vec<String> {
    let url_regex = Regex::new(r"(\|\|)?https?://(?:www\.)?twitter\.com/\S+(\|\|)?").unwrap();

    let mut fixed_urls = Vec::new();

    for cap in url_regex.captures_iter(content) {
        let url = cap.get(0).unwrap().as_str();
        let fixed_url = url.replace("twitter.com", "fxtwitter.com");
        let is_spoiler = cap.get(1).is_some() && cap.get(2).is_some();

        println!("is_spoiler: {}", is_spoiler);
        if is_spoiler {
            fixed_urls.push(format!("||{}||", fixed_url));
        } else {
            fixed_urls.push(fixed_url);
        }
    }

    fixed_urls
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://twitter.com/ProZD/status/1714895861208293467";
    const FIXED_URL: &str = "https://fxtwitter.com/ProZD/status/1714895861208293467";
    const FIXED_SPOILER: &str = "||https://fxtwitter.com/ProZD/status/1714895861208293467||";

    #[test]
    fn pass_through_normal_text() {
        let fixed = fix_msg("this\nis\nsome\nstuff\ncone\nwould\nsay");
        assert_eq!(fixed, Vec::<String>::new());
    }

    #[test]
    fn fix_solo_url() {
        let fixed = fix_msg(TEST_URL);
        assert_eq!(fixed, vec![FIXED_URL]);
    }

    #[test]
    fn fix_url_in_text() {
        let content = format!("this is text\n{}\nmore content", TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL]);
    }

    #[test]
    fn fix_multiple_urls() {
        let content = format!("{}\n{}", TEST_URL, TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL, FIXED_URL]);
    }

    #[test]
    fn handle_spoilers() {
        let content = format!("shhhh ||{}||", TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_SPOILER]);
    }

    #[test]
    fn handle_multiple_spoilers() {
        let content = format!("shhhh ||{}||\nshh||{}|| ", TEST_URL, TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_SPOILER, FIXED_SPOILER]);
    }

    #[test]
    fn handle_broken_spoilers() {
        let content = format!("shhhh ||secret|| {}||", TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL]);
    }
}
