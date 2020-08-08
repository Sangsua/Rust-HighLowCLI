use rand::Rng;
use std::io::stdin;

fn main() {
    //high low game
    //needed values

    let mut rng = rand::thread_rng();

    // println!("{}",number);
    //gameloop

    let mut game_loop_done; 
    'gameloop : loop {
        let number = rng.gen_range(1, 100) as i32;
        println!("please insert a number between 1 and 100");
        'inputloop : loop {
            game_loop_done = false;
            let mut input = String::new();
            stdin().read_line(&mut input).expect("failed to read_line");
            let input_trimmed = input.trim();
            let input_parsed = input_trimmed.parse::<i32>();
            match input_parsed {
                Ok(n) => {
                    println!("Number {} is ok" , n);
                }
                Err(error) => {
                    println!("number is not ok => error : {}",error);
                    break;

                }
            }
            match input_trimmed.parse::<i32>() {
                Ok(n) => {
                    if n == number {
                        println!("gj");
                        println!("");
                        game_loop_done = true;
                    } else if n < number {
                        println!("Number is bigger!" );
                    } else if n > number {
                        println!("number is smaller");
                    }
                }
                Err(error) => println!(" error: {}", error),
            }
            if game_loop_done{
                println!("do you want to continue?");
                println!("Input yes or y to continue");
                println!("Else this will terminate");

                let mut continue_input = String::new();
                stdin().read_line(&mut continue_input).expect("Failed to read_line");
                let continue_input_uppercase = continue_input.to_uppercase();
                let continue_input_uppercase_trimmed = continue_input_uppercase.trim();
                match continue_input_uppercase_trimmed {
                   "YES"|"Y" =>break 'inputloop,
                   _ => break 'gameloop,
                }
            }
        }
    }
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
