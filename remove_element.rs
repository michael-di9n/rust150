
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // retain is a method that keeps only values that satisfy a predicate
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}
