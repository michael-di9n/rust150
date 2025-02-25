// 1 index decrement the 1 index
// Requires constant space
// Two pointers method the idea is greedy if the smallest and largest is two big then the only direction is to reduce the largest
// if the smallest and largest is too small then the only way is to move the smallest 
// Actually the idea is built on previous answers influence the next if num[l] + num[r] > ans then since num[l + 1] > num[l] then num[l + 1] + num[r] 
// therefore search for proper pairing with l + 1 starts at index < r
// Simple algorithm but easy to make off by 1 mistakes and type mistakes

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0 as usize;
        let mut r = numbers.len() - 1;
        while l < r {
            if numbers[l] + numbers[r] == target {
                return vec![(l + 1) as i32, (r + 1) as i32];
            } else if numbers[l] + numbers[r] > target {
                r -= 1;
            } else {
                l += 1;
            }
        }
        vec![-1, -1]
    }
}