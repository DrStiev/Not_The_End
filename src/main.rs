use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    prelude::*,
};
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
enum TokenKind {
    Success,
    Complication,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TokenKind::Complication => write!(f, "Complicazione"),
            TokenKind::Success => write!(f, "Successo"),
        }
    }
}

impl Distribution<TokenKind> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TokenKind {
        match rng.random_range(0..2) {
            0 => TokenKind::Success,
            1 => TokenKind::Complication,
            _ => unreachable!(),
        }
    }
}

fn generate_bag(success: i32, difficulties: i32) -> Vec<TokenKind> {
    let size: usize = (success + difficulties) as usize;
    let mut vec = Vec::with_capacity(size);
    // add traits
    for _ in 0..success {
        vec.push(TokenKind::Success);
    }
    // add failure
    for _ in 0..difficulties {
        vec.push(TokenKind::Complication);
    }
    vec
}

fn draw_token(bag: &mut Vec<TokenKind>, draw: usize) -> Vec<TokenKind> {
    let mut vec = Vec::with_capacity(bag.capacity());
    for _ in 0..draw {
        let (i, _) = bag.iter().enumerate().choose(&mut rand::rng()).unwrap();
        vec.push(bag.swap_remove(i));
    }
    vec
}

fn main() {
    println!("Benvenuti, io sono un amichevole aiuto per giocare a Not The End online!");
    let mut input = String::new();

    // Infinite loop for call to action
    loop {
        input.clear(); // First clear the String. Otherwise it will keep adding to it
        print!("1. Quanti TRATTI vuoi usare? ");
        io::stdout().flush().unwrap(); // FLUSH del buffer
        io::stdin().read_line(&mut input).unwrap();
        let traits = input.trim().parse::<i32>().unwrap();

        input.clear(); // First clear the String. Otherwise it will keep adding to it
        print!("2. Quanto è DIFFICILE la prova? ");
        io::stdout().flush().unwrap(); // FLUSH del buffer
        io::stdin().read_line(&mut input).unwrap();
        let difficulty = input.trim().parse::<i32>().unwrap();

        input.clear(); // First clear the String. Otherwise it will keep adding to it
        print!("3. Quanti TOKEN vuoi PESCARE (1-4)? ");
        io::stdout().flush().unwrap(); // FLUSH del buffer
        io::stdin().read_line(&mut input).unwrap();
        let draw = input.trim().parse::<i32>().unwrap();

        // Generate bag of token
        let mut bag = generate_bag(traits, difficulty);
        let result = draw_token(&mut bag, draw as usize);
        for t in result {
            print!("{t} ");
        }
        println!();

        input.clear(); // First clear the String. Otherwise it will keep adding to it
        print!("4. Vuoi rischiare (Y/n)? ");
        io::stdout().flush().unwrap(); // FLUSH del buffer
        io::stdin().read_line(&mut input).unwrap();
        let risk = input.trim();
        if risk == "Y" || risk == "y" {
            let result = draw_token(&mut bag, (5 - draw) as usize);
            for t in result {
                print!("{t} ");
            }
            println!();
        }
    }
}
