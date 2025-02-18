// Chain of throught get the first word and just slowly decrement the index
// Edge case: empty string, string complex prefix
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first_w = &strs[0];
        let mut prefix_end = first_w.len();

        for w in &strs {
            if w.len() == 0 {
                prefix_end = 0;
            }
            for i in 0..w.len() {
                if i >= prefix_end {
                    break;
                }
                if w.as_bytes()[i] != first_w.as_bytes()[i] {
                    prefix_end = i;
                    break;
                }
                if i == w.len() - 1 {
                    prefix_end = i + 1;
                }
            }
        }

        first_w[0..prefix_end].to_string()
    }
}
