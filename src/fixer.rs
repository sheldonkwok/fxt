use rand::seq::SliceRandom;
use regex::Regex;
use std::env;

pub fn fix_msg(content: &str) -> Vec<String> {
    let spoilers = get_spoilers(content);
    let mut fixes = Vec::new();

    fixes.extend(fix_twitter_urls(content, &spoilers));
    fixes.extend(fix_tiktok_urls(content, &spoilers));
    fixes.extend(fix_instagram_urls(content, &spoilers));

    fixes
}

fn get_spoilers(content: &str) -> Vec<(usize, usize)> {
    let mut indexes = Vec::new();
    let mut start = 0;

    while start < content.len() {
        if let Some(pos) = content[start..].find("||") {
            indexes.push(start + pos);
            start = start + pos + 2;
        } else {
            break;
        }
    }

    let mut pairs = Vec::new();
    if indexes.len() < 2 {
        return pairs;
    }

    for chunk in indexes.chunks(2) {
        if chunk.len() == 2 {
            pairs.push((chunk[0], chunk[1]));
        }
    }

    pairs
}

fn wrap_spoiler(fix: &str, match_start: usize, spoilers: &[(usize, usize)]) -> String {
    for &(start, end) in spoilers {
        if match_start > start && match_start < end {
            return format!("||{}||", fix);
        }
    }
    fix.to_string()
}

fn fix_twitter_urls(content: &str, spoilers: &[(usize, usize)]) -> Vec<String> {
    let re = Regex::new(r"https://(twitter|x)\.com/\w+/status/\d+").unwrap();
    let domain_re = Regex::new(r"(twitter|x)\.com").unwrap();

    re.find_iter(content)
        .map(|m| {
            let fix = domain_re.replace(m.as_str(), get_twitter_fixer()).to_string();
            wrap_spoiler(&fix, m.start(), spoilers)
        })
        .collect()
}

fn fix_tiktok_urls(content: &str, spoilers: &[(usize, usize)]) -> Vec<String> {
    let re = Regex::new(r"https://(?:www\.|vm\.|vt\.)?tiktok\.com/[^\s|]*").unwrap();
    let domain_re = Regex::new(r"tiktok\.com").unwrap();

    re.find_iter(content)
        .map(|m| {
            let fix = domain_re.replace(m.as_str(), "tnktok.com").to_string();
            wrap_spoiler(&fix, m.start(), spoilers)
        })
        .collect()
}

fn fix_instagram_urls(content: &str, spoilers: &[(usize, usize)]) -> Vec<String> {
    let re = Regex::new(r"https://(?:www\.)?instagram\.com/[^\s|]*").unwrap();
    let domain_re = Regex::new(r"instagram\.com").unwrap();

    re.find_iter(content)
        .map(|m| {
            let fix = domain_re.replace(m.as_str(), "kkinstagram.com").to_string();
            wrap_spoiler(&fix, m.start(), spoilers)
        })
        .collect()
}

const DEFAULT_TWITTER_FIX: &str = "fxtwitter.com";

fn get_twitter_fixer() -> &'static str {
    let secret = env::var("SECRET_FIXERS").unwrap_or_default();
    let fixers: Vec<&str> = secret.split(',').filter(|s| !s.is_empty()).collect();

    if fixers.is_empty() {
        return DEFAULT_TWITTER_FIX;
    }

    // Leak the string so we can return a &'static str — only called with a small set of env values
    let chosen = fixers.choose(&mut rand::thread_rng()).unwrap();
    Box::leak(chosen.to_string().into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://twitter.com/ProZD/status/1714895861208293467";
    const FIXED_URL: &str = "https://fxtwitter.com/ProZD/status/1714895861208293467";

    fn fixed_spoiler() -> String {
        format!("||{}||", FIXED_URL)
    }

    #[test]
    fn pass_through_normal_text() {
        let fixed = fix_msg("this\n  is \n  some \n  stuff\n  cone \n  would\n  say");
        assert_eq!(fixed, Vec::<String>::new());
    }

    #[test]
    fn fix_solo_url() {
        let fixed = fix_msg(TEST_URL);
        assert_eq!(fixed, vec![FIXED_URL]);
    }

    #[test]
    fn fix_url_in_text() {
        let content = format!("this is text\n  {}\n  more content", TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL]);
    }

    #[test]
    fn fix_multiple_urls() {
        let content = format!("{}\n  {}", TEST_URL, TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL, FIXED_URL]);
    }

    #[test]
    fn handle_spoilers() {
        let content = format!("shhhh ||{}||", TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![fixed_spoiler()]);
    }

    #[test]
    fn handle_multiple_spoilers() {
        let content = format!("shhhh ||{}||\n  shh||{}|| ", TEST_URL, TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![fixed_spoiler(), fixed_spoiler()]);
    }

    #[test]
    fn handle_broken_spoilers() {
        let content = format!("shhhh ||secret|| {}||", TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL]);
    }

    // TikTok tests
    const TIKTOK_TEST_URL: &str = "https://www.tiktok.com/@username/video/1234567890";
    const TIKTOK_FIXED_URL: &str = "https://www.tnktok.com/@username/video/1234567890";

    const TIKTOK_SHORT_URL: &str = "https://vm.tiktok.com/ZMjKL1234/";
    const TIKTOK_SHORT_FIXED: &str = "https://vm.tnktok.com/ZMjKL1234/";

    fn tiktok_fixed_spoiler() -> String {
        format!("||{}||", TIKTOK_FIXED_URL)
    }

    #[test]
    fn fix_tiktok_url() {
        let fixed = fix_msg(TIKTOK_TEST_URL);
        assert_eq!(fixed, vec![TIKTOK_FIXED_URL]);
    }

    #[test]
    fn fix_tiktok_short_url() {
        let fixed = fix_msg(TIKTOK_SHORT_URL);
        assert_eq!(fixed, vec![TIKTOK_SHORT_FIXED]);
    }

    #[test]
    fn fix_tiktok_url_in_text() {
        let content = format!("check this out {} amazing video", TIKTOK_TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![TIKTOK_FIXED_URL]);
    }

    #[test]
    fn fix_multiple_tiktok_urls() {
        let content = format!("{} and {}", TIKTOK_TEST_URL, TIKTOK_SHORT_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![TIKTOK_FIXED_URL, TIKTOK_SHORT_FIXED]);
    }

    #[test]
    fn handle_tiktok_spoilers() {
        let content = format!("secret video ||{}||", TIKTOK_TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![tiktok_fixed_spoiler()]);
    }

    #[test]
    fn fix_both_twitter_and_tiktok_urls() {
        let content = format!("{} and {}", TEST_URL, TIKTOK_TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL, TIKTOK_FIXED_URL]);
    }

    // Instagram tests
    const INSTAGRAM_TEST_URL: &str = "https://www.instagram.com/p/ABC123/";
    const INSTAGRAM_FIXED_URL: &str = "https://www.kkinstagram.com/p/ABC123/";

    const INSTAGRAM_REEL_URL: &str = "https://www.instagram.com/reel/XYZ789/";
    const INSTAGRAM_REEL_FIXED: &str = "https://www.kkinstagram.com/reel/XYZ789/";

    fn instagram_fixed_spoiler() -> String {
        format!("||{}||", INSTAGRAM_FIXED_URL)
    }

    #[test]
    fn fix_instagram_url() {
        let fixed = fix_msg(INSTAGRAM_TEST_URL);
        assert_eq!(fixed, vec![INSTAGRAM_FIXED_URL]);
    }

    #[test]
    fn fix_instagram_reel_url() {
        let fixed = fix_msg(INSTAGRAM_REEL_URL);
        assert_eq!(fixed, vec![INSTAGRAM_REEL_FIXED]);
    }

    #[test]
    fn fix_instagram_url_in_text() {
        let content = format!("check this out {} cool post", INSTAGRAM_TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![INSTAGRAM_FIXED_URL]);
    }

    #[test]
    fn fix_multiple_instagram_urls() {
        let content = format!("{} and {}", INSTAGRAM_TEST_URL, INSTAGRAM_REEL_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![INSTAGRAM_FIXED_URL, INSTAGRAM_REEL_FIXED]);
    }

    #[test]
    fn handle_instagram_spoilers() {
        let content = format!("secret post ||{}||", INSTAGRAM_TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![instagram_fixed_spoiler()]);
    }

    #[test]
    fn fix_twitter_tiktok_and_instagram_urls() {
        let content = format!("{} and {} and {}", TEST_URL, TIKTOK_TEST_URL, INSTAGRAM_TEST_URL);
        let fixed = fix_msg(&content);
        assert_eq!(fixed, vec![FIXED_URL, TIKTOK_FIXED_URL, INSTAGRAM_FIXED_URL]);
    }
}
