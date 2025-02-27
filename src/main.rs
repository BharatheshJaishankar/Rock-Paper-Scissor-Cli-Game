use dialoguer;
use rand::prelude::IndexedRandom;

fn main() {
    let mut rng = rand::rng();
    loop {
        println!("Rock, Paper, Scissors!");
        let options = vec!["Rock", "Paper", "Scissor"];
        let selection = dialoguer::Select::new()
            .with_prompt("Choose your weapon")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        let input = options[selection];
        let decision = options.choose(&mut rng).unwrap();
        println!("The computer chose {}", decision);
        if input == "Rock" && *decision == "Scissor" {
            println!("You win!");
            break;
        } else {
            if input == "Rock" && *decision == "Rock" {
                println!("Its a tie!\nTry again\n");
                continue;
            } else {
                if input == "Paper" && *decision == "Rock" {
                    println!("You win!");
                    break;
                } else {
                    if input == "Paper" && *decision == "Paper" {
                        println!("Its a tie!\nTry again\n");
                        continue;
                    } else {
                        if input == "Scissor" && *decision == "Paper" {
                            println!("You win!");
                            break;
                        } else {
                            if input == "Scissor" && *decision == "Scissor" {
                                println!("Its a tie!\nTry again\n");
                                continue;
                            } else {
                                println!("You lose!");
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
