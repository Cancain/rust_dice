use rand::prelude::*;
use std::io;

enum Choice {
    Exit,
    Roll
}

fn main() {
    let mut done = false;

    while !done {
        let choice = menu_greeting();
        match choice {
            Choice::Exit => done = true,
            Choice::Roll => main_loop()
        }
    }
}

fn main_loop() {
    let dice_roll = roll();
    print_unicode(dice_roll);
}

fn menu_greeting() -> Choice {
    println!("Make a choice: E[x]it | [r]oll");

    let mut done = false;
    let mut choice = Choice::Exit;
    
    while !done {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Sorry, something went wrong");

        match input.to_lowercase().trim() {
            "x" => {
                choice = Choice::Exit;
                done = true;
            }
            "r" => {
                choice = Choice::Roll;
                done = true;
            }
            _ => done = false
        }
    }

    choice
}

fn roll() -> i8{
    let mut rng = rand::thread_rng();
    let roll: i8 = rng.gen_range(1, 7);
    roll
}

fn print_unicode(number: i8) {
    let unicode_dice;
    match number {
        1 => unicode_dice = String::from("\u{2680}"),
        2 => unicode_dice = String::from("\u{2681}"),
        3 => unicode_dice = String::from("\u{2682}"),
        4 => unicode_dice = String::from("\u{2683}"),
        5 => unicode_dice = String::from("\u{2684}"),
        6 => unicode_dice = String::from("\u{2685}"),
        _ => {
            unicode_dice = String::from("Error!");
            println!("{}", number);
        }
    }
    println!("You rolled: {}", unicode_dice);
}

