// day9
// Iterate through nums and update shortest i.e., the shortest distance to get to pos i
// Only continue if shortest[i] is a value otherwise we cannot reach and quit
// If shortest[i] is a value then update the next values we can reach and havn't previosuly reached from shortest[i] to shortest[i] + 1 
// !Make it efficient by recording the right most element traversed O(n) solution
// Return the last element of shortest

use std::cmp::max;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 || nums.len() == 1 {
            return 0;
        }

        let mut shortest = vec![-1; nums.len()];
        shortest[0] = 0;

        let mut end: usize = 0;
        let mut right_most: usize = 0;

        for i in 0..nums.len() {
            if shortest[i] == -1 {
                return -1; 
            }

            end = i + nums[i] as usize;
            for j in right_most..end {
                shortest[j + 1] = shortest[i] + 1;
                if j + 1 == nums.len() - 1 {
                    return shortest[j + 1];
                }
            }
            right_most = max(right_most, end);
        }
        shortest[nums.len() - 1]

    }
}
// Accepted 100

// Refactor to be O(1) space complexity - Calculate a boundary for each level between left and right; then update left = right and right = furthest and increment a jump count
