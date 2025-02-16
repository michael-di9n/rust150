impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let  ss = s.split_whitespace();
        if let Some(res) = ss.last() {
            res.len() as i32;
        }
        -1
    }
}