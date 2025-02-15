use std::collections::HashMap;
// Attempt to subtract M until we cant
// Greedy solution if the remaining number starts with a 9 or 4 then we do the special thing
// How to find the first positions n / (10 pow len(n) - 1) then remainder but division is pretty expenseive
// Alternatively can just list out all 10 options for 10's 100's and 1000's and then index into array; or a map with the string that inlcudes the 4 and 9 combos
// it aint pretty but it works 
impl IntRoman {
    pub fn int_to_roman(num: i32) -> String {
        // let mut mapping: HashMap<char, i32>  = HashMap::new();
        // mapping.insert(1, 'I');
        // mapping.insert(5, 'V');
        // mapping.insert(10, 'X');
        // mapping.insert(50, 'L');
        // mapping.insert(100, 'C');
        // mapping.insert(500, 'D');
        // mapping.insert(1000, 'M');
        let mut roman: Vec<char> = vec![];
        let mut num = num;
        while num >= 1000 {
            num -= 1000;
            roman.push('M')
        }

        while num > 0 {
            if num >= 900 {
                num -= 900;
                roman.push('C');
                roman.push('M');
            } else if num >= 500 {
                num -= 500;
                roman.push('D');
            } else if num >= 400 {
                num -= 400;
                roman.push('C');
                roman.push('D');
            } else if num >= 100 {
                num -= 100;
                roman.push('C');
            } else if num >= 90 {
                num -= 90;
                roman.push('X');
                roman.push('C');
            } else if num >= 50 {
                num -= 50;
                roman.push('L');
            } else if num >= 40 {
                num -= 40;
                roman.push('X');
                roman.push('L');
            } else if num >= 10 {
                num -= 10;
                roman.push('X');
            } else if num == 9 {
                num -= 9;
                roman.push('I');
                roman.push('X');
            } else if num >= 5 {
                num -= 5;
                roman.push('V');
            } else if num == 4 {
                num -= 4;
                roman.push('I');
                roman.push('V');
            } else if num > 0 {
                num -= 1;
                roman.push('I')
            }
        }
        roman.iter().collect()
    }
}


// string ones[] = {"","I","II","III","IV","V","VI","VII","VIII","IX"};
// string tens[] = {"","X","XX","XXX","XL","L","LX","LXX","LXXX","XC"};
// string hrns[] = {"","C","CC","CCC","CD","D","DC","DCC","DCCC","CM"};
// string ths[]={"","M","MM","MMM"};