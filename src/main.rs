extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("\n\t** MaxTable **");
    println!("Trouve les 10 bonnes réponses!\n\n");

    let mut first_v = vec![0;10];
    let mut second_v = vec![0;10];
    let mut total: u32 = 0;

    //initialise les vecteurs avec 10 chiffres de 1 à 9
    for i in 0..10 {
        first_v[i] = rand::thread_rng().gen_range(1, 11);
        second_v[i] = rand::thread_rng().gen_range(1, 11);
    }

    //Boucle pour faire 10 opérations
    for i in 0..10 {
        println!("({}) {}x{}=?", i+1, first_v[i], second_v[i]);

        let mut guess = String::new();
        let guess_int: u32; //sera utilisé pour transformer String en Int
        let result: u32 = first_v[i]*second_v[i];
        
        //Boucle sans fin tant que l'utilisateur n'a pas entré un chiffre
        loop {
            //Lecture de la ligne entrée par l'utilisateur
            io::stdin().read_line(&mut guess)
                .ok()
                .expect("Impossible de lire le résultat");

            //Test pour voir si c'est un Int -> le cas échéant on sort de la boucle
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

        //Comparaison entre le résultat et ce qui a été entré par l'utilisateur
        if result == guess_int {
            println!(":) Bravo ! \n");
            total = total + 1;
        } else {
            println!(":( Perdu, le résultat est {} \n", result);
        }

       
    }
    //Fin et affichage du score
    println!("Tu as un score de de {}/10", total);

    
}