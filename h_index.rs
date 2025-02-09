// According to the definition of h-index on Wikipedia:
// The h-index is defined as the maximum value of h such that the given researcher has published at least
// h papers that have each been cited at least h times.

// CoT O(n2) where for each number we count if h is greater starting from the highest
// Sort the papers first and then determine if pos i is valid since at pos i if it is h then the remaining is more than h element it has that h index
// Handle two conditions - citations is high but limited by papers, papers is low and limited by citations

use std::cmp::min;
use std::cmp::max;

// O(n) time and space
// Create a citation bucket array that mesure the number papers with citation i O(n)
// Do a cumulative sweep from the end to the start tracking the number of papers that 
// which tracks the number of papers with atleast citation_buclet[i] citations
// Compare it with condition and update the citation count (just limited by citation)

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut citation_bucket = vec![0; n + 1];
        // Iterate through citations.iter() 'refence' no move and use refernce value also no move &v
        for &v in citations.iter() {
            if (v as usize) > n {
                citation_bucket[n]+= 1;
            } else {
                citation_bucket[v as usize] += 1;
            }
        }
        let mut count = 0;
        let mut h_index = 0;
        for i in (0..=n).rev() {
            count += citation_bucket[i];
            // i is the current reference count
            // count is the number of papers that have reference count > i
            if count >= i { 
                h_index = i;
                break;
            }
        }

        h_index as i32
    }   
}


// Accepted O(n * logn) time and O(n) space
impl Solution1 {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut nums = citations.clone();
        nums.sort();
        let n = citations.len();
        let mut h_index = 0;
        for i in 0..n {
            h_index = max(h_index, min(nums[i], (n-i) as i32));
        }
        h_index
    }   
}