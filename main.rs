use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess=String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_)=>{
            println!("you guessed {} ",guess);
        },
        Err(e)=>{
            println!("OOPs,Something went wrong");
        }
    }

    let secret_number=rand::thread_rng().gen_range(1..101);
    println!("The secret number is : {}",secret_number);
}