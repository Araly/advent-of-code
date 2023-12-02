use std::{collections::HashMap, env, fs};

fn main() {
    let input = get_input();
    let calibration_strings: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;
    for calibration_string in calibration_strings {
        sum += extract_calibration_value(calibration_string.to_string())
    }
    println!("total = {sum}")
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("should read the file");

    return contents;
}

fn extract_calibration_value(input: String) -> u32 {
    println!("{input}");

    if input.contains("\n") {
        panic!("calibration value input should not contain newlines")
    }

    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    let mut first_digit_found = false;

    for i in 0..input.len() {
        let (digit, is_found) = to_elf_digit(input.clone(), i);
        if is_found {
            last_digit = digit;
            if !first_digit_found {
                first_digit = digit;
                first_digit_found = true;
            }
        }
    }

    let calibration_value = first_digit * 10 + last_digit;

    return calibration_value;
}

fn to_elf_digit(input: String, i: usize) -> (u32, bool) {
    let characters: Vec<char> = input.chars().collect();
    if characters[i].is_digit(10) {
        println!(
            "=> at {} found {}",
            i,
            characters[i].to_digit(10).expect("should be digit"),
        );
        return (characters[i].to_digit(10).expect("should be a digit"), true);
    }

    let digits = populate_digits();
    let input_from_i = &input[i..];
    for digit in digits {
        let position: usize = match input_from_i.find(&digit.1) {
            None => continue,
            Some(position) => position,
        };
        if position == 0 {
            println!("=> at {} found {}", position, digit.1);
            return (digit.0, true);
        }
    }

    return (0, false);
}

fn populate_digits() -> HashMap<u32, String> {
    let mut digits = HashMap::new();

    digits.insert(1, String::from("one"));
    digits.insert(2, String::from("two"));
    digits.insert(3, String::from("three"));
    digits.insert(4, String::from("four"));
    digits.insert(5, String::from("five"));
    digits.insert(6, String::from("six"));
    digits.insert(7, String::from("seven"));
    digits.insert(8, String::from("eight"));
    digits.insert(9, String::from("nine"));

    return digits;
}
