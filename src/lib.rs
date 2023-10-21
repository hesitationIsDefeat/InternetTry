use std::cmp::Ordering::*;
use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
extern "C" {
    fn prompt(message: &str) -> String;
    fn alert(message: &str);

}

pub fn run_game() {
    let secret_number: u8 = (Math::random() * 100.0) as u8 + 1;
    let mut guess_amount: usize = 0;
    alert("Welcome");
    loop {
        let guess = prompt("Please enter your guess:");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                alert("Please enter a number");
                continue;
            }
        };
        guess_amount += 1;
        match guess.cmp(&secret_number) {
            Less => alert("Guess higher..."),
            Greater => alert("Guess lower..."),
            Equal => {
                alert("Congrats. You guessed the number in {guess_amount} tries!");
                break;
            }
        }
    }
}