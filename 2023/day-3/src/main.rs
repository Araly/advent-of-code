use std::{env, fs};

fn main() {
    let input = get_input();
    let numbers = parse_numbers(&input);
    let grid = parse_symbols(&input);

    let mut sum = 0;
    for number in numbers {
        let is_part_number = is_part_number(&number, &grid);
        if is_part_number {
            sum += number.number;
        }
    }
    println!("total = {sum}");
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("should read the file");

    return contents;
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    x: usize,
    y: usize,
}

fn parse_numbers(input: &String) -> Vec<PartNumber> {
    let mut numbers = vec![];
    let rows: Vec<&str> = input.split("\n").collect();
    for y in 0..rows.len() {
        let row: Vec<char> = rows[y].chars().collect();
        let mut number_start = 0;
        let mut number_present = false;
        for x in 0..row.len() {
            if row[x].is_digit(10) && !number_present {
                // found the start of a number
                number_present = true;
                number_start = x;
            } else if (!row[x].is_digit(10) || x == row.len().saturating_sub(1)) && number_present {
                // found the end of a number
                let row = row.iter().collect::<String>();
                let number: u32 = row[number_start..x]
                    .parse::<u32>()
                    .expect("string should be a number");
                numbers.push(PartNumber {
                    number,
                    x: number_start,
                    y,
                });
                number_present = false;
            }
        }
    }

    return numbers;
}

fn parse_symbols(input: &String) -> Vec<Vec<bool>> {
    let mut symbols: Vec<Vec<bool>> = vec![];
    let rows: Vec<&str> = input.split("\n").collect();
    for y in 0..rows.len() {
        let row: Vec<char> = rows[y].chars().collect();
        let mut symbols_row: Vec<bool> = vec![false; row.len()];
        for x in 0..row.len() {
            if !row[x].is_digit(10) && row[x] != '.' {
                symbols_row[x] = true;
            }
        }
        symbols.push(symbols_row);
    }

    return symbols;
}

fn is_part_number(number: &PartNumber, grid: &Vec<Vec<bool>>) -> bool {
    let y_start = number.y.saturating_sub(1);
    let y_end = number.y.saturating_add(1);
    let x_start = number.x.saturating_sub(1);
    let x_end = number.x.saturating_add(number.number.to_string().len());

    for y in y_start..(y_end + 1).min(grid.len()) {
        for x in x_start..(x_end + 1).min(grid[y].len()) {
            if grid[y][x] {
                // print!("y:{y},x:{x} = true | ");
                println!("{}: searched between y:({y_start}..{y_end}), x:({x_start}..{x_end}), found y:{y},x:{x}", number.number);
                return true;
                // } else {
                // print!("y:{y},x:{x} = false | ");
            }
        }
        // println!();
    }

    println!(
        "{}: searched between y:({y_start}..{y_end}), x:({x_start}..{x_end})",
        number.number
    );
    return false;
}
