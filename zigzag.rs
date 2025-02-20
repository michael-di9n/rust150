// Generate each vector which depends on the current row
// Use vertical traversal with the dir variable and identify each row
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut curr_row: i32 = 0;
        let mut dir = -1;
        let mut res = vec![String::new(); num_rows as usize];
        for c in s.chars() {
            res[curr_row as usize].push(c);
            if curr_row == 0 || (curr_row == num_rows - 1) {
                dir = -dir;
            }
            curr_row += dir;
        }
        res.concat()
    }
}
