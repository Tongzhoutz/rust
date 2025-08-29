use std::io; // preclude: https://doc.rust-lang.org/stable/std/prelude/index.html

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let mut guess = String::new(); // mutable; new: associated function
                                   // created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
