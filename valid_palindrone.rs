use regex::Regex;

struct padlindrome;
impl padlindrome {
    pub fn is_palindrome(s: String) -> bool {
        let rgx = Regex::new("[^a-zA-Z0-9]").unwrap(); // unwrap since we are sure it is valid
        let clean_s = rgx.replace_all(&s, "").to_lowercase(); // sanitize the entire sting
        let chars : Vec<char> = clean_s.chars().collect(); // convert to a vector using collect of chars to allow indexing. rust does not allow direct indexing into strs since
        // we need to convert to utf8
        
        for i in 0..chars.len() / 2 {
            if chars[i] != chars[ chars.len() - i - 1] {
                return false;
            }
        }
        true
    }
}