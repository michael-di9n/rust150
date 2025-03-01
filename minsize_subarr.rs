// Kadanes algorithm of subarrays
// Greedy algorithm 
// Grow the subarray until it reaches target then attempt to shrink it
// Kadanes will find all subarrays that have atleast target since it starts the pivot left most and goes through all combos

// Valid is greater than or equal
struct MinSub;
impl MinSub {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut cum_sum = 0;
        let mut l = 0;
        let mut r = 0;
        let n = nums.len();
        let mut best_len = 0;

        while r < n {
            while r < n && cum_sum < target {
                cum_sum += nums[r];
                r += 1;
            }
            // This also handles the expanding phase which is just a single one
            // The shrinking phase produces the most correct cum_sums
            while l < n && cum_sum >= target {
                if best_len == 0 || r - l < best_len {
                    best_len = r - l;
                }

                cum_sum -= nums[l];
                l += 1;
            }
        }

        best_len as i32
    }
}


#[cfg(test)]
mod tests {

    use super::MinSub;

    #[test]
    fn simple_case() {
        assert_eq!(MinSub::min_sub_array_len(7, vec![2,3,1,2,4,3]), 2);
    }

    #[test]
    fn edge_case() {
        assert_eq!(MinSub::min_sub_array_len(11, vec![1,2,3,4,5]), 3)
    }
}