// day7
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        
        let mut profit = 0;
        let mut cost = prices[0];
        let mut total_profit = 0;
        for i in 1..prices.len() {
            if prices[i] > cost {
                total_profit += prices[i] - cost;
            }
            cost = prices[i];
        }
        total_profit
    }
}


// CoT
// Similar to single buy and sell case except we can buy sell infinite
// The idea here is to sell the dip +ve -> -ve, buy the upwards trend -ve -> +ve (buy - hold - sell)
// Kadanes algorithm: subset array maximum value - start a new one when sum is <= 0; Similar but we also record all the positive values before starting a new one
// If the sum goes down we just start a new one - and keep track of the min
// Using Kadanes - convert prices to delta array i.e., vec[i+1] - vec[i]; then only take the positive subarrays and as sson as it dips start a new one