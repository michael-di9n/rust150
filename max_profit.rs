use std::cmp::max;

// Day6
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut curr_max = 0;
        for price in prices.iter() {
            if *price < min {
                min = *price;
            }
            curr_max = max(curr_max, price - min);
        }
        curr_max
    }
}