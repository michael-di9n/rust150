

//day12 accepted O(n) space

struct ProductSum;
impl ProductSum {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![1];
        }
        let mut prefix : Vec<i32> = vec![nums[0]];
        let mut suffix : Vec<i32> = vec![nums[n-1]];
        for i in 1..n {
            prefix.push(prefix[i-1] * nums[i]);
            suffix.push(suffix[i-1] * nums[n - i - 1]);
        }
        let mut product : Vec<i32> = vec![suffix[n - 2]];
        for i in 1..n-1 {
            product.push(prefix[i-1] * suffix[n - 1 - i - 1]);
        } 
        product.push(prefix[n-2]);
        product
    }   
}


// O(1) Chain of thought = track the value manually solve at the instance
// The output array doesn't count then it simple no - then just store the values in out
// Do one pass to calculate left side products, then another pass to do right side products


pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![1];
    }
    let mut product : Vec<i32> = vec![1; nums.len()];
    let mut left = 1;
    for i in 1..n {
        left *= nums[i-1];
        product[i] = left;
    } 

    let mut right = 1; // i = n-2 .. 0
    for i in (0..n-1).rev() {
        right *= nums[i + 1];
        product[i] *= right;
    }
    product
}   




// Test cases
#[cfg(test)]
mod tests {
    use super::ProductSum;

    #[test]
    fn test_basic_cases() {
        assert_eq!(ProductSum::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(ProductSum::product_except_self(vec![4, 5, 1, 8, 2]), vec![80, 64, 320, 40, 160]);
    }

    #[test]
    fn test_with_zeros() {
        assert_eq!(ProductSum::product_except_self(vec![1, 0, 3, 4]), vec![0, 12, 0, 0]);
        assert_eq!(ProductSum::product_except_self(vec![0, 0, 3, 4]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(ProductSum::product_except_self(vec![5]), vec![1]);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(ProductSum::product_except_self(vec![3, 4]), vec![4, 3]);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(ProductSum::product_except_self(vec![1000, 2000, 3000]), vec![6000000, 3000000, 2000000]);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(ProductSum::product_except_self(vec![-1, -2, -3, -4]), vec![-24, -12, -8, -6]);
    }
}