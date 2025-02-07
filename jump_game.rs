
//day8


impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 0 || nums.len() == 1{
            return true;
        }
        let mut gas = nums[0];
        for i in 1..nums.len() {
            gas -= 1;
            if gas < 0 {
                return false;
            }  if nums[i] > gas {
                gas = nums[i];
            } 
        }
        true
    }
}

impl Solution_slow {
    pub fn dp(curr: usize, nums: &Vec<i32>, visited: &mut Vec<bool>) -> bool {
        if curr == (nums.len() - 1) {
            return true;
        }
        
        if visited[curr] == true {
            return false
        } 

        for i in 1..(nums[curr] as usize + 1) {
            if Self::dp(curr + i, nums, visited) {
                return true
            }
        }
        visited[curr] = true;
        false
    }

    pub fn can_jump(nums: Vec<i32>) -> bool {
      if nums.len() == 0 || nums.len() == 1 {
        return true;
      }
      let mut start = nums[0];
      let mut visited = vec![false; nums.len()];

       Self::dp(0, &nums, &mut visited)
    }
}

// CoT
// backtracking + memo = dp
// cannot reach if there exist a zero
// memo[i] = False (hans't been explored) memo[i] = True has been explored and can't reach end
// start at nums[i] and cotinue until hit a zero 