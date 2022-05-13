use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number 🔑🔑🔑");
    let secret = rand::thread_rng().gen_range(0..101);
    // println!("The secret number is: {}", secret);

    loop {
        println!("Input ur guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🤬🤬🤬 not a number 🤬🤬🤬");
                println!("⏭ try again");
                continue
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("🔻 Too small!"),
            Ordering::Greater => println!("🔺 Too big!"),
            Ordering::Equal => {
                println!("🏆🏆🏆 You win!");
                break;
            },
        }


        println!("Guessed: {}", guess);
    }
}
