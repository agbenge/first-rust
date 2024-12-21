/* create random number game for 1 to 5  rust */

pub fn play()  {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let secret_number = 3;
    println!("The secret number is: {}", secret_number);
    if guess.trim().parse::<i32>().unwrap() == secret_number {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}   // end of play function

// greeting function
pub fn greeting() -> String {
    return String::from("Hello, world!");
}   // end of greeting function