
use std::collections::HashMap;

// There is another rule that is roman numerals that is large number always go before small unless it means subtraction... huh.. so
// VM is not valid for 1005 ITS ALWAYS MV
impl Roman {
    pub fn roman_to_int(s: String) -> i32 {
        let mut mapping: HashMap<char, i32>  = HashMap::new();
        mapping.insert('I', 1);
        mapping.insert('V', 5);
        mapping.insert('X', 10);
        mapping.insert('L', 50);
        mapping.insert('C', 100);
        mapping.insert('D', 500);
        mapping.insert('M', 1000);
        // Also consider subtraction case IV and IX ; XL and XC ; CD and CM

        let mut number = 0;
        let mut hold = 0;
        for c in s.chars() {
            // int implements the copy trait because its a primitive type so dereferencing doenst take ownership; but use copied since this works for everything not like strings
            let val = mapping.get(&c).copied().unwrap();

            if hold == 0 {
                hold = val;
            } else {
                if hold == 5 || hold == 50 || hold == 500 {
                    number += hold;
                    hold = val;
                } else if val / hold == 5 || val / hold == 10 {
                    number += val - hold;
                    hold = 0;
                } else {
                    number += hold;
                    hold = val;
                }
            }
        }
        number + hold
    }

    // Only process the previous if we know the current one.
}