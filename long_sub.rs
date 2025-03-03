// Greedy approach get two pointers
// Since if c occurs at i and c occurs at j then the longest substring will be between those
// if we encounter a duplicate then shorten until its not longer included
// Any longest string will be between two duplicates or the end
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut best_len = 0;
        let mut set: HashSet<char> = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        
        // Use a for loop since we go through the entire array
        for r in 0..chars.len() {
            while set.contains(&chars[r]) {
                set.remove(&chars[l]);
                l += 1;
            }
            set.insert(chars[r]);
            best_len = best_len.max(r - l + 1);
        }
    
        best_len as i32
    }
}