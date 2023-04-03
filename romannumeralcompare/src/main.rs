fn roman_to_int(roman: &str) -> i32 {
    let mut total = 0;
    let mut prev_value = 0;

    let value = |c| {
        match c {
            'M' => 1000,
            'D' => 500,
            'C' => 100,
            'L' => 50,
            'X' => 10,
            'V' => 5,
            'I' => 1,
            _ => 0,
        }
    };

    for c in roman.chars().rev() {
        let current_value = value(c);
        if current_value < prev_value {
            total -= current_value;
        } else {
            total += current_value;
        }
        prev_value = current_value;
    }

    total
}

fn compare_roman_numerals(input: &[(String, String)]) -> Vec<bool> {
    input
        .into_iter()
        .map(|(r1, r2)| roman_to_int(&r1) > roman_to_int(&r2)) // Change '<' to '>'
        .collect()
}

fn main() {
    let roman_numerals = vec![
        (String::from("I"), String::from("I")),
        (String::from("II"), String::from("I")),
        (String::from("I"), String::from("II")),
        (String::from("IV"), String::from("III")),
        (String::from("MMM"), String::from("MMCCCCLXXXXIV")),
        (String::from("MMMCCCLXXXIV"), String::from("MMCCCCLXXXXIV")),
        (String::from("MMMMMCCCCLXXXIV"), String::from("MMCCCCLXXXXIV")),
    ];
    let results = compare_roman_numerals(&roman_numerals);
    for ((r1, r2), result) in roman_numerals.iter().zip(&results) {
        println!("{} > {}: {}", r1, r2, result);
    }
}