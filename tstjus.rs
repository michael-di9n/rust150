// Attempt to build each line of text with the set width
// Determine how many words we can have in that line
// i.e., If its more then slit the extra space evently that is left side gets more

use std::cmp::max;

struct Just;
impl Just {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut words_len = 0;
        let mut words_to_join: Vec<String> = vec![];
        let mut result: Vec<String> = vec![];
        for word in words.iter() {
            if word.len() + words_len + words_to_join.len() > max_width as usize {
                // Maximum length reached
                // 0 if there is only 1 word
                let spaces_req = words_to_join.len() - 1; 
                let rem = max_width as usize - words_len;
                let gap = if spaces_req > 0 { rem / spaces_req } else { 0 };
                let mut additional = if spaces_req > 0 { rem % spaces_req } else { 0 };
                let mut temp_line: Vec<String> = vec![];
                for (i, w) in words_to_join.iter().enumerate() {
                    temp_line.push(w.to_string());
                    if i < spaces_req {
                        if additional > 0 {
                            additional -= 1;
                            temp_line.push(" ".repeat(gap + 1));
                        } else {
                            temp_line.push(" ".repeat(gap));
                        }
                    } else if spaces_req == 0 {
                        temp_line.push(" ".repeat(rem));
                    }
                }
                result.push(temp_line.concat());
                words_to_join.clear();
                words_len = 0;
            }
            words_to_join.push(word.to_string());
            words_len += word.len();
        }
        if words_to_join.len() > 0 {
            let mut lastline = words_to_join.join(" ");
            lastline.push_str(&" ".repeat(max_width as usize - lastline.len()));
            result.push(lastline);
        }
        result
    }
}


#[cfg(test)]

mod tests {
    use super::Just;

    #[test]
    fn simple_case() {
        assert_eq!(Just::full_justify(vec!["This", "is", "an", "example", "of", "text", "justification."].iter()
        .map(|s| s.to_string())
        .collect(), 16),  vec!["This    is    an",
        "example  of text","justification.  "]);

        
        assert_eq!(Just::full_justify(vec!["What","must","be","acknowledgment","shall","be"].iter()
        .map(|s| s.to_string())
        .collect(), 16),  vec![
        "What   must   be",
        "acknowledgment  ",
        "shall be        "
        ]);

        
        assert_eq!(Just::full_justify(vec!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter()
        .map(|s| s.to_string())
        .collect(), 20),  vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
          ]);
    }

    #[test]
    fn simple_case1() {

        assert_eq!(Just::full_justify(vec!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter()
        .map(|s| s.to_string())
        .collect(), 20),  vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
          ]);
    }

    #[test]
    fn simple_case2() {
        assert_eq!(Just::full_justify(vec!["What","must","be","acknowledgment","shall","be"].iter()
        .map(|s| s.to_string())
        .collect(), 16),  vec![
        "What   must   be",
        "acknowledgment  ",
        "shall be        "
        ]);
    }
}