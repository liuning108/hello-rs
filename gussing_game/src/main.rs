use std::cmp::Ordering;
use std::io;
use std::ptr::null;
use rand::Rng;

//The fn syntax declares a new function, the parentheses, (), indicate there are no parameters, and the curly bracket, {, starts the body of the function.
fn main() {
    println!("Guess the number!");
    let secret_number =rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        //we’ll create a variable to store the user input, like this:
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}！！！",guess);
        match guess.cmp(&secret_number) {
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break
            }
        }
    }





}
