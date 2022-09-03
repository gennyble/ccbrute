use std::fmt::Write;

use card_validate::Validate;

fn main() {
    println!("Enter your card number. Replace unknown digits with an underscore");
    println!("Use as many spaces as you like, or none at all. They'll be removed.");
    println!("EX:\n\t4266 12__ __34 5678\n");

    let mut cc = String::new();
    std::io::stdin().read_line(&mut cc).unwrap();

    let mut working_cc = vec![];
    let mut unknown_indicies = vec![];

    // validate input
    for ch in cc.chars() {
        if ch == '_' {
            unknown_indicies.push(working_cc.len());
            working_cc.push(ch);
            continue;
        }

        if ch.is_whitespace() {
            continue;
        }

        if ch.is_digit(10) {
            working_cc.push(ch);
            continue;
        }

        println!("'{}' is an invalid character", ch);
        return;
    }

    if unknown_indicies.len() == 0 {
        println!("No underscored found so no work to do!");
        return;
    }

    let mut max = 1;
    for _ in unknown_indicies.iter() {
        max *= 10;
    }

    println!("Starting! Max guesses: {}", max);
    let mut valid_numbers = vec![];

    let mut str = String::with_capacity(unknown_indicies.len());
    for guess in 0..max {
        str.clear();
        write!(
            &mut str,
            "{:0width$}",
            guess,
            width = unknown_indicies.len()
        )
        .unwrap();

        let mut iter = str.chars();

        for idx in &unknown_indicies {
            working_cc[*idx] = iter.next().unwrap();
        }

        let card_string = String::from_iter(working_cc.iter());
        println!("guessing {}", card_string);
        match Validate::from(&card_string) {
            Ok(_) => {
                valid_numbers.push(card_string);
            }
            Err(_) => (),
        }
    }

    if valid_numbers.len() == 0 {
        println!("Could not find any valid numbers!");
    } else {
        println!("Found {} valid numbers:", valid_numbers.len());
        for num in valid_numbers {
            println!("{}", num);
        }
    }
}
