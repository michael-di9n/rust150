// Sort the ratings give all children with the lowest rating 1 candy
// Give all children with 2nd lowest rating 1 candy if it is not adjacent to the lowest rating otherwise they get 2
// gove rating lv 3 1 or max(adjacent) + 1 if the are different from if its the same value then assume that has a 0
// start at 0 O(n^2) solution will this meet the constraints 10^8 pc cycles
// Generate a map of rating = positions

//day14
// By default binary heap is max heap
use std::collections::{HashMap, BinaryHeap};
use std::cmp::max;
struct CandySimple;
impl CandySimple {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy_distrubution: Vec<i32> = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i-1] {
                candy_distrubution[i] = 1 + candy_distrubution[i-1];
            } 
        }
        for i in (0..ratings.len()-1).rev() {
            if ratings[i] > ratings[i+1] {
                candy_distrubution[i] = max(candy_distrubution[i], candy_distrubution[i+1] + 1);
            }
        }
        candy_distrubution.iter().sum()
    }
}

struct Candy;
impl Candy {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut map : HashMap<i32, Vec<usize>> = HashMap::new();
        let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut candy_distrubuted : Vec<i32> = vec![0; n];
        for i in 0..n {
            // Use negative to convert to min heap
            map.entry(ratings[i]).or_insert(vec![]).push(i);
            min_heap.push(-ratings[i]);
        }
        while !min_heap.is_empty() {
            let rate = -min_heap.pop().unwrap();
            for &i in map.get(&rate).unwrap_or(&vec![]).iter() {
                // Carefull here remember to use candy_distrubuted as the value that we want 1 + of
                let left = if i == 0 || ratings[i-1] == rate { 0 } else { candy_distrubuted[i-1]};
                let right = if i+1 >= n || ratings[i+1] == rate { 0 } else { candy_distrubuted[i+1]};
                candy_distrubuted[i] = 1 + max(left, right);
            }
        }
        candy_distrubuted.iter().sum()
    }
}
// Accepted solution O(n)

#[cfg(test)]

mod tests {
    use super::Candy;

    #[test]
    fn simple_case() {
        assert_eq!(Candy::candy(vec![1, 2, 1]), 4)
    }

    #[test]
    fn basic_case() {

    }

    #[test]
    fn degen_case() {

    }

}