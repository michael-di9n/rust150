struct WaterAlot;
// Greedy approach
// Test if we use the height at i then find the right most maximum height at the left of i that is greater than itself
// Then move the pointer that is limiting the water by one- two pointer tweak the limiter because we need height and distance the highest one might not be the best
// Future solutions are based on previous solutions if moving from i to i+1 imagine case where we are always measuring the best with one of the sticks
// Idea is for each of the height we need to find the right most that is greater or equal imagine we find r now if we increment l if l is less than or equal to l-1 then this is also valid

use std::cmp::{max, min};
impl WaterAlot {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0 as usize;
        let mut r = height.len() - 1;
        let mut curr_max = 0;
        while l < r {
            curr_max = max(curr_max, (r - l) as i32 * min(height[l], height[r])); 
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        curr_max
    
    }
}