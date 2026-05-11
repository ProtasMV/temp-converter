use std::io::{self, Write};

fn main() {
    loop { 
        println!("1)конвертация с (°C) в (Фарангейты), 2)конвертация с (Фарангейтов) в (°С)");
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
                println!("Неверный запрос, повторите попытку");
                continue;
            }
        }
    }
}

fn conversion(signs: (&str, char, &str), action: bool) {
    let (sign1, sign2, main_sign) = signs;

    println!();
    print!("Введите температуру в ({main_sign}): ");
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
        Err(er) => println!("Произошла ошибка! {er}")
    }
    data
}

fn flush() {
    match io::stdout().flush() {
        Ok(()) => {},
        Err(er) => {println!("Произошла ошибка! {er}")}
    }
}

fn parse_f32(data: String, main_sign: &str) -> f32 {
    let data: f32 = match data.trim().parse() {
        Ok(data) => {data},
        Err(er) => {
            loop {
                println!();
                println!("Произошла ошибка! {er}, попробуйте ещё раз");
                print!("Введите температуру в ({main_sign}): ");
                flush();

                let data = input();
                let data = parse_f32(data, main_sign);
                break data;
            }
        },
    };
    data
}

fn user_continue() -> bool{
    loop {
        println!();
        print!("Прододжить? 1)Да, 2)Нет: ");
        flush();

        let user_input = input();
        match user_input.trim().to_lowercase().as_str() {
            "1"|"да"|"y" => {break true},
            "2"|"нет"|"n" => {break false},
            _=> {println!("Неверный запрос, повторите попытку"); continue}
        }
    }
}