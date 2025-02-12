// day13
// Notice that if we could not go from i to j then starting at i+1 we start at an equal or lower fuel count thus we cannot go past j
// Therefore the next start should be j+1

struct SolutionM;
impl SolutionM {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut stock = 0;
        // if the total_stock is not enough then its impossible
        let mut total_stock = 0;
        let n = gas.len();
        let mut start = 0;
        
        for i in 0..n {
            total_stock += gas[i] - cost[i];
            stock += gas[i] - cost[i];
            if stock < 0 {
                start = i + 1;
                stock = 0;
            }
        }
        if total_stock >= 0 {
            return start as i32;
        }
        -1
    }

}



// Simple algorithm works but not very effieicnt
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut net_cost : Vec<i32> = vec![]; 
    for i in 0..n {
        net_cost.push(gas[i] - cost[i]);
    }
    
    for start in 0..n {
        let mut stock = 0;
        let mut k = 0;
        while k < n && stock + net_cost[(start + k) % n] >= 0 {
            stock += net_cost[(start + k) % n];
            k += 1;
        }
        if k == n {
            return start as i32;
        }
    }
    -1
}

// Proof of over using over complex algo
// the increasing length operation will never result in bad subarray since we manually check before growing the front 
// However the popping back may lets so lets prove by contracdtion that its valid 
// let [a:n] be the case were we need to remove back; i.e., net sum < 0 then let [a+k:n] be the final array such that netsum >= 0
// Assume there exist a [a+k:a+k+m] where a+k+m < n, be the negative sum subarray this implies we had a net positive subarray from [a:a+k-1] 
// since [a:a+k+m] was valid when we were growing the front however, this implies we subtracted a positive amount from  net sum < 0 to reach netsum >= 0 
// which is a impossible. Thus by contradiction there is no such negative subarray can arrise due to the popping left operation 
struct Gas;
impl Gas {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // This vector is the net_cost from going from i: node i to node i+1
        let mut net_cost : Vec<i32> = vec![]; 
        let n = gas.len();
        for i in 0..n {
            net_cost.push(gas[i] - cost[i]);
        }
        for i in 0..n {
            net_cost.push(net_cost[i]);
        }

        // Run Kadanes algorithm + Caterpie Algorithm if we have start == end then end or if we have start == start again and didn't finish
        let mut back : usize = 0;
        let mut front : usize = 0;
        let mut curr_stock : i32 = 0;
        let mut k = 0;
        while front < 2*n {
            while back < n && curr_stock + net_cost[front] < 0 {
                back += 1;
                if front > (back - 1) {
                    curr_stock -= net_cost[back -1];
                    k -= 1;
                } else {
                    front = back;
                }
            }

            if back == n { 
                return  -1;
            }
            
            while front < 2*n && curr_stock + net_cost[front] >= 0 {
                curr_stock += net_cost[front];
                front = front + 1;
                k += 1;
                
                // we have found a valid loop
                if k == n {
                    return (back % n) as i32;
                }
            }   
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Gas;
    #[test]
    fn test_simple_case() {
        assert_eq!(Gas::can_complete_circuit(vec![5, 5, 5], vec![1, 1, 1]), 0);
    }

    #[test]
    fn test_basic_case() {
        assert_eq!(Gas::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
        assert_eq!(Gas::can_complete_circuit(vec![2,3,4], vec![3,4,3]), -1);
    }

    #[test]
    fn test_one_case() {
        assert_eq!(Gas::can_complete_circuit(vec![5], vec![1]), 0);
        assert_eq!(Gas::can_complete_circuit(vec![0], vec![1]), -1);
    }
}