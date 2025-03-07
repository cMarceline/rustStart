use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is {secret_number}");


    loop {

      let guess: u32 = guess();
      println!("You guessed: {guess}");

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        },
      }

    }
}

fn guess() -> u32 {

  println!("Please input your guess.");
  let mut guess = String::new();

  io::stdin()
   .read_line(&mut guess)
   .expect("Failed to read line");
    
  let guess: u32 = guess
   .trim()
   .parse()
   .expect("Please type a number");

  return guess
}
