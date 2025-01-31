// Day 1
use std::collections::HashSet;

struct Solution;


impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // Convert nums to a set and check if the size decreased
        let mut mutable_nums =  nums.clone();
        mutable_nums.sort();
        mutable_nums.dedup();
        mutable_nums.len() < nums.len()
    }
}

// Passes 5.94% and 98.34% inefficient since we can immediately stop when a single duplicate is detected
// TODO modify remove dedup() and simple iterate check the previous number