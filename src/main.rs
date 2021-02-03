use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;
fn main() {
    //high low game
    //needed values

    let mut rng = rand::thread_rng();

    let mut game_loop_done : bool; 
    let mut lives :i32;
    // game start
    // game start
    // game start
    // game start
    // game start
    // game start

    'gameloop : loop {
        lives = 8;
        let number = rng.gen_range(1, 100) as i32;
        println!("please insert a number between 1 and 100");
        println!("you have {} lives",lives);
        println!();

        'inputloop : loop {
            game_loop_done = false;
            let mut input = String::new();
            stdin().read_line(&mut input).expect("failed to read_line");
            let input_trimmed = input.trim();
            // let input_parsed = input_trimmed.parse::<i32>();
            // match input_parsed {
            //     Ok(n) => {
            //         println!("Number {} is ok" , n);
            //         println!();
            //     }
            //     Err(error) => {
            //         println!("number is not ok => error : {}",error);
            //         break;

            //     }
            // }
            match input_trimmed.parse::<i32>() {
                Ok(n) => {
                    //                    if n == number {
                    //                        println!("gj");
                    //                        println!();
                    //                        game_loop_done = true;
                    //                    } else if n < number {
                    //                        println!("Number is bigger!" );
                    //                        lives -= 1;
                    //                        println!("Lives: {}",lives );
                    //                    } else if n > number {
                    //                        println!("number is smaller");
                    //                        lives -= 1;
                    //                        println!("Lives: {}",lives );
                    //                    }

                    match n.cmp(&number) {
                        Ordering::Greater => {
                            println!("the number you are searching is lower");
                            lives -=1;
                            println!("Lives: {}",lives );
                        },
                        Ordering::Less => {
                            println!("the number you are searching is bigger");
                            lives -=1;
                            println!("Lives: {}",lives );

                        },
                        Ordering::Equal => {
                            println!();
                            println!("gj");
                            game_loop_done = true;
                        }

                    }
                }
                Err(error) => {
                    println!("no number to parse: {}", error);
                    println!("please try again");
                    println!("Lives: {}",lives );
                    println!();
                    continue
                }
            }
            if lives == 0 {
                println!();
                println!("you died in a fire");
                game_loop_done= true;
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

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
