

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // Implement Boyer-Moore vote algorithm
        let mut candidate = nums[0];
        let mut count = 0;
        // This consumes the nums vector
        for v in nums {
            if v == candidate {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    candidate = v;
                    count = 1;
                }
            }
        }
        candidate
    }
}
// Accepted