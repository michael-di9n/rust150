impl Solution {
    pub fn reverse_words(s: String) -> String {
        let strings: Vec<&str> = s.split_whitespace().collect(); // perferred over split(' ')
                                                                 // rev operation works on iterations, map returns an iterator
        let rev_strings: Vec<String> = strings.iter().rev().map(|&word| word.to_string()).collect();
        rev_strings.join(" ")
    }
}
// Notes: map takes a reference each element of the string.iter() which is a &str 'string slice' i.e., &&str
// in the closure we are dereference by saying we are 'passing by reference so dereference it implicitly please' i.e., |&word|
// this is done by pattern matching
// now instead of working with a pointer we can work with the word &str directly, and call the to_string() method
// which creates an owned String instance
