use std::{env, fs};

fn main() {
    let input = get_input();
    let games: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;
    for game in games {
        if game == "" {
            continue;
        }
        let game = parse_game(game.to_owned());
        let is_possible = is_possible(&game);

        if is_possible {
            sum += game.id;
        }
    }
    println!("total: {sum}");
}

fn print_game(game: Game) {
    let mut result: String = "Game ".to_owned() + &game.id.to_string() + ":";
    for run in game.runs {
        if run.red != 0 {
            let color = " ".to_owned() + &run.red.to_string() + " red,";
            result.push_str(&color);
        }
        if run.green != 0 {
            let color = " ".to_owned() + &run.green.to_string() + " green,";
            result.push_str(&color);
        }
        if run.blue != 0 {
            let color = " ".to_owned() + &run.blue.to_string() + " blue,";
            result.push_str(&color);
        }
        result.push_str(";")
    }
    println!("{}", result);
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("should read the file");

    return contents;
}

struct Game {
    id: u32,
    runs: Vec<Run>,
}

struct Run {
    red: u32,
    green: u32,
    blue: u32,
}

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn parse_game(input: String) -> Game {
    let (input_game, input_runs) = input.split_once(":").expect("couldn't split input");
    let (_, id) = input_game
        .split_once(" ")
        .expect("couldn't split input_game");
    let id = id
        .parse::<u32>()
        .expect("couldn't parse id from String to u32");
    let input_runs: Vec<&str> = input_runs.split(";").collect();

    let mut runs: Vec<Run> = vec![];

    for run in input_runs {
        let colors: Vec<&str> = run.split(",").collect();
        let mut run = Run {
            red: 0,
            green: 0,
            blue: 0,
        };
        for color in colors {
            let color = color.trim();
            let (value, key) = color
                .split_once(" ")
                .expect("couldn't split the value and the key from the color");
            let value = value
                .parse::<u32>()
                .expect("couldn't parse color value from &str to u32");
            match key {
                RED => run.red = value,
                GREEN => run.green = value,
                BLUE => run.blue = value,
                &_ => panic!("couldn't match color key to one of the colors"),
            }
        }
        runs.push(run);
    }

    let game = Game { id, runs };

    return game;
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn is_possible(game: &Game) -> bool {
    for run in &game.runs {
        if run.red > MAX_RED || run.green > MAX_GREEN || run.blue > MAX_BLUE {
            return false;
        }
    }
    return true;
}
