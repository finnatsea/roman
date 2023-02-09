use std::io;

const ROMAN_NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

fn to_roman_numeral(mut number: u32) -> String {
    let mut result = String::new();
    for (value, symbol) in ROMAN_NUMERALS.iter() {
        while number >= *value {
            result.push_str(symbol);
            number -= *value;
        }
    }
    result
}

fn main() {
    println!("Enter a number to convert to a Roman numeral:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid input. Please enter a positive integer.");
            return;
        }
    };

    if number == 0 {
        println!("Error: The number must be greater than zero.");
        return;
    }

    let roman_numeral = to_roman_numeral(number);
    println!(
        "The Roman numeral equivalent of {} is: {}",
        number, roman_numeral
    );
}

/* -------------------------------------------------- */
/*                        TESTS                       */
/* -------------------------------------------------- */

#[test]
fn test_to_roman_numeral() {
    assert_eq!(to_roman_numeral(1), "I");
    assert_eq!(to_roman_numeral(4), "IV");
    assert_eq!(to_roman_numeral(9), "IX");
    assert_eq!(to_roman_numeral(10), "X");
    assert_eq!(to_roman_numeral(40), "XL");
    assert_eq!(to_roman_numeral(50), "L");
    assert_eq!(to_roman_numeral(90), "XC");
    assert_eq!(to_roman_numeral(100), "C");
    assert_eq!(to_roman_numeral(400), "CD");
    assert_eq!(to_roman_numeral(500), "D");
    assert_eq!(to_roman_numeral(900), "CM");
    assert_eq!(to_roman_numeral(1000), "M");
    assert_eq!(to_roman_numeral(1954), "MCMLIV");
    assert_eq!(to_roman_numeral(1990), "MCMXC");
}
