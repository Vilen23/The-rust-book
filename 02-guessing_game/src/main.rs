use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); //mut--> means the variable is mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //Result --> Varients like Ok and Err .expect is one of the method on an instance of result

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
//Some key takes-->
//references are immutable by default too
