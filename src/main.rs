extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("\n\t** MaxTable **");
    println!("Trouve les 10 bonnes réponses!\n\n");

    let mut first_v = vec![0;10];
    let mut second_v = vec![0;10];
    let mut total: u32 = 0;

    for i in 0..10 {
        first_v[i] = rand::thread_rng().gen_range(1, 11);
        second_v[i] = rand::thread_rng().gen_range(1, 11);
    }

    for i in 0..10 {
        println!("({}) {}x{}=?", i+1, first_v[i], second_v[i]);

        let mut guess = String::new();
        let guess_int: u32;
        let result: u32 = first_v[i]*second_v[i];

        loop {
            io::stdin().read_line(&mut guess)
                .ok()
                .expect("Impossible de lire le résultat");

            match guess.trim().parse::<u32>() {
                Ok(num) => {
                    guess_int = num;
                    break;
                },
                Err(_) => {
                    guess = String::new();
                    println!("[!] Entre un nombre");
                    continue;
                }
            };
        }

        //let guess: Option<u32> = guess.trim().parse().ok();
        //let guess: u32 = match guess { Some(num) => num };

        if result == guess_int {
            println!(":) Bravo ! \n");
            total = total + 1;
        } else {
            println!(":( Perdu, le résultat est {} \n", result);
        }

       
    }
    println!("Tu as un score de de {}/10", total);

    
}