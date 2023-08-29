use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number() {
    println!("Guess the number!");
    let _no: u32 = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is {_no}");
    
    loop {
        println!("Please input your Guess.:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&_no) {
            Ordering::Less => println!("The number is too small!"),
            Ordering::Greater => println!("The number is too big!"),
            Ordering::Equal => {
                println!("Correct Answer! You Win!");
                break;
            }
        }
    }
}
