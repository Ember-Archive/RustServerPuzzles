fn compare_roman_numerals(input: &[(String, String)]) -> Vec<bool> {
    let roman_order: [char; 7] = ['M', 'D', 'C', 'L', 'X', 'V', 'I'];

    input
        .iter()
        .map(|(r1, r2)| {
            let mut r1_iter = r1.chars().peekable();
            let mut r2_iter = r2.chars().peekable();

            loop {
                let r1_char = r1_iter.peek();
                let r2_char = r2_iter.peek();

                match (r1_char, r2_char) {
                    (Some(c1), Some(c2)) => {
                        if c1 == c2 {
                            r1_iter.next();
                            r2_iter.next();
                        } else {
                            let c1_index = roman_order.iter().position(|&c| c == *c1).unwrap();
                            let c2_index = roman_order.iter().position(|&c| c == *c2).unwrap();
                            return c1_index > c2_index;
                        }
                    }
                    (Some(_), None) => return false,
                    (None, Some(_)) => return true,
                    (None, None) => return false,
                }
            }
        })
        .collect()
}

fn main() {
    let roman_numerals = vec![
        (String::from("I"), String::from("I")),
        (String::from("I"), String::from("II")),
        (String::from("II"), String::from("II")),
        (String::from("V"), String::from("IV")),
        (String::from("MDCLXV"), String::from("MDCLXVI")),
        (String::from("MM"), String::from("MMCCCCLXXXXIV")),
        (String::from("MMMMMCCCCLXXXIV"), String::from("MMCCCCLXXXXIV")),
    ];
    let results = compare_roman_numerals(&roman_numerals);
    for ((r1, r2), result) in roman_numerals.iter().zip(results) {
        println!("{} > {}: {}", r1, r2, result);
    }
}

/*numcompare("I", "I") => false
numcompare("I", "II") => true
numcompare("II", "I") => false
numcompare("V", "IIII") => false
numcompare("MDCLXV", "MDCLXVI") => true
numcompare("MM", "MDCCCCLXXXXVIIII") => false */