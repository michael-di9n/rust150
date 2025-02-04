//day5
    // Chain of thought 
    // Rotate k to the right take modulus n rotate n is same
    // position i in rotated vector is n - r + i in nums vector where r is the rotation
    impl Solution {

        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let n = nums.len();
            let mut rotated: Vec<i32> = vec![];
            let r = (k as usize) % nums.len();
            for i in 0..nums.len() {
                rotated.push(nums[(n - r + i) % n]);
            }
            for i in 0..rotated.len() {
                nums[i] =rotated[i];
            }
        }
    }