use std::io::{self, Write};
use colored::*;

const SYS_COLOR: Color = Color::TrueColor { r: 255, g: 255, b: 0 };
const AUTOR_COLOR: Color = Color::TrueColor {r: 255, g: 165, b: 0};

fn main() {
println!("{}", "Created by: @ProtasMV".color(AUTOR_COLOR).bold());
    loop { 
        println!("{}", "1)Convert (°C) to Fahrenheit, 2)Convert Fahrenheit to (°C)".color(SYS_COLOR));
        let user_answer = input();
        
        match user_answer.trim() {
            "1" => {
                let signs = ("°C", 'F', "°C");
                let to_f_action = true;
                conversion(signs, to_f_action);
                if !user_continue() {break}
            }
            "2" => {
                let signs = ("°C", 'F', "F");                
                let to_f_action = false;
                conversion(signs, to_f_action);
                if !user_continue() {break}                              
            }
            _ => {
                println!("Invalid input, please try again");
                continue;
            }
        }
    }
}

fn conversion(signs: (&str, char, &str), action: bool) {
    let (sign1, sign2, main_sign) = signs;

    println!();
    let temp = format!("Enter temperature in ({main_sign}): ");
    print!("{}", temp.color(SYS_COLOR));
    flush();

    let user_data = input();
    let user_data = parse_f32(user_data, main_sign);

    match action {
        true => {
            println!("{}{sign2}", (user_data * 1.8) + 32.0)
        },
        false => {
            println!("{}{sign1}", (user_data - 32.0) / 1.8 )
        },    
    }    
}

fn input() -> String {
    let mut data = String::new();
    match io::stdin().read_line(&mut data) {
        Ok(_) => {},
        Err(er) => println!("An error occurred! {er}")
    }
    data
}

fn flush() {
    match io::stdout().flush() {
        Ok(()) => {},
        Err(er) => {println!("An error occurred! {er}")}
    }
}

fn parse_f32(mut data: String, main_sign: &str) -> f32 {
    loop {
        match data.trim().parse() {
            Ok(data) => {return data},
            Err(er) => {
                println!();
                println!("An error occurred! {er}, please try again");
                let temp = format!("Enter temperature in ({main_sign}): ");
                print!("{}", temp.color(SYS_COLOR));
                flush();
                
                data = input();
            },
        };
    }
}

fn user_continue() -> bool{
    loop {
        println!();
        print!("{}", "Continue? 1)Yes, 2)No: ".color(SYS_COLOR));
        flush();

        let user_input = input();
        match user_input.trim().to_lowercase().as_str() {
            "1"|"yes"|"y" => {break true},
            "2"|"no"|"n" => {break false},
            _=> {println!("Invalid input, please try again"); continue}
        }
    }
}