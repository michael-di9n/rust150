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
    
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let n = nums.len();
        
        if n < 3 {
            return result; // Not enough elements
        }
    
        nums.sort(); // Sort in place
    
        for i in 0..n {
            // Skip duplicates for the first element
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
    
            let target = -nums[i];
            let (mut left, mut right) = (i + 1, n - 1);
    
            while left < right {
                let sum = nums[left] + nums[right];
                if sum == target {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    
                    // Skip duplicates for second and third elements
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
    
                    left += 1;
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        
        result
    }
}