use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    arr: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        // Return an instance of RandomizedSet
        RandomizedSet {
            map : HashMap::new(),
            arr : Vec::new()
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        } 
        self.map.insert(val, self.arr.len() );
        self.arr.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.map.get(&val) {
            let last = self.arr.pop().unwrap();
            // If index == self.arr.len() => index == previous n-1 i.e., last element then not need to 'perserve' last element
            if index < self.arr.len() {
                self.arr[index] = last;
                // This will overwrite if it already exist
                self.map.insert(last, index); 
            }
            self.map.remove(&val);
            return true;
        }
        false
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..self.arr.len());
        self.arr[n]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */