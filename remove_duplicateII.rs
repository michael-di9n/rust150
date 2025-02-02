impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // go through the array and mark if a possition is going to be removed
        if nums.len() < 2 {
            return nums.len() as i32
        }
        // increment a removed count
        // encounter a non removed element then shift that element by the removed count forward
        // rust may require two passes anyways use an index

        let mut top = 1;
        let mut curr_num = nums[0];
        let mut curr_count = 1;
        for i in 1..nums.len() {
            if curr_num == nums[i] {
                curr_count += 1;
            } else {
                curr_num = nums[i];
                curr_count = 1;
            }

            if curr_count < 3 {
                nums[top] = nums[i];
                top += 1;
            }
        }

        top as i32
        
    }
}


// Accepted solution in O(N) time and O(1) space