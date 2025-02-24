// Requerement is to have the same chars in t in the same order - implies find char then find char + 1 in the rest of t
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        
        let mut i = 0;
        let mut j = 0;
        while i < s.len() {
            let mut found = false;
            while j < t.len() {
                if s.as_bytes()[i] == t.as_bytes()[j] {
                    found = true;
                    j += 1;
                    break;
                }
                j += 1;
            }
            if found {
                i += 1;
            } else {
                break;
            }
        }
        i == s.len()
    }
}