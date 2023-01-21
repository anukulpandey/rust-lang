use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guessing Game");
    println!("built with ❤️  by 0xanukul");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Make a guess in range (1,100) :");

    loop{
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Please enter a value");
        
        println!("You guessed {}",guess);

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {continue},
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","too low".red()),
            Ordering::Equal => {
                println!("{}","You won".green());
                break;
            },
            Ordering::Greater =>println!("{}","too high".red()),
        }
    }

}
