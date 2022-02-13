/*A wordle program*/
use colored::*;
use rand::{thread_rng, Rng};
use std::cmp::PartialEq;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
//the diffrent colors a letter in the guess can be. Anything that isn't a match can be assumed as white.
#[derive(Debug, PartialEq)]
enum Color {
    Yellow,
    Green,
}
fn valid_guess(guess: &String)->bool{
    //fast check
    if guess.len()!=5{
        return false;
    }
    //*loops through the valid words

    return true;
}
fn print_guess(guess: &String, matches: &Vec<(u8, Color)>) {
    //loops through the letter of the user's guess
    for i in 0..guess.len() {
        //this flag dictate wether or not a match was found
        let mut found = false;
        //look for matches
        for j in matches {
            //prints the match with the correct colour
            if j.0 == i as u8 {
                //the current letter
                let letter = String::from(guess.clone().chars().nth(i).unwrap());
                if j.1 == Color::Green {
                    print!("{}", letter.green());
                } else {
                    print!("{}", letter.yellow());
                }
                io::stdout().flush().unwrap();
                //tells the loop a match was found
                found = true;
                break;
            }
        }
        //prints the letter normally if no match was found
        if !found {
            print!("{}", guess.clone().chars().nth(i).unwrap());
            io::stdout().flush().unwrap();
        }
        io::stdout().flush().unwrap();
    }
    println!("")
}
fn check_guess(awnser: &String, word: &String) -> Vec<(u8, Color)> {
    let mut matches: Vec<(u8, Color)> = Vec::new();
    //we copy the original guess and awnser and remove the matches from this value so that we can search for yellows
    let mut trimmed_guess = word.clone();
    let mut trimmed_awnser = awnser.clone();
    //*searches for green matches
    //we use a loop so that i can be decramented
    let mut i = 0;
    //becuse we mutate the awnser and consequntly change it's length, we save the offset we changed the length by
    let mut offset = 0;
    loop {
        //checks if the word is finished
        if i >= trimmed_awnser.len() {
            break;
        }
        if trimmed_guess.chars().nth(i) == trimmed_awnser.chars().nth(i) {
            //adds the match
            matches.push((i as u8 + offset, Color::Green));
            //removes the letter from the check
            trimmed_guess.remove(i);
            trimmed_awnser.remove(i);
            //updates teh offset
            offset += 1
        } else {
            i += 1;
        }
    }
    //checks if the word was guessed
    if matches.len() == awnser.len() {
        return matches;
    }
    //*searched for yellow matches
    //resets the offset for the yellows
    offset = 0;
    for i in 0..trimmed_guess.len() {
        //the current letter
        let found = trimmed_guess.chars().nth(i - offset as usize);
        match found {
            Some(letter) => {
                if trimmed_awnser.contains(letter) {
                    //a letter might already be green and this flag checks that
                    let mut taken = false;
                    for j in &matches {
                        //looks for an identical index
                        if j.0 == i as u8 {
                            taken = true;
                            break;
                        }
                    }
                    if taken {
                        continue;
                    }
                    //adds the match
                    matches.push((i as u8, Color::Yellow));
                    //removes the letter from the check
                    trimmed_guess.remove(i - offset as usize);
                    //searches for the letter and removes it
                    trimmed_awnser
                        .remove(trimmed_awnser.chars().position(|c| c == letter).unwrap());
                    //updates the offset
                    offset += 1;
                }
            }
            None => continue,
        }
    }
    return matches;
}
fn main() -> io::Result<()> {
    //*exracts the words
    let file = File::open(Path::new("words.txt"))?;
    let reader = BufReader::new(&file);
    //gets the contents as a Vec
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    //*chooses a word
    let mut rng = rand::thread_rng();
    let number: usize = rng.gen_range(0..lines.len());
    //the awnser the player will have to guess
    let awnser = lines.get(number).unwrap();
    //the guesses the user has said so far
    let mut guesses: Vec<(String, Vec<(u8, Color)>)> = Vec::new();
    println!("\x1B[2J\x1B[1;1H");
    //*game loop
    for _ in 0..5 {
        //*finds matches
        //gets the guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        //removes the \n at the end of the lien
        guess.pop();
        //removes teh \r at the end of the lien
        guess.pop();
        //adds the guess to the Vec
        guesses.push((guess.clone(), check_guess(awnser, &guess)));
        //clears the screen and prints the guesses
        print!("\x1B[2J\x1B[1;1H");
        for i in &guesses {
            print_guess(&i.0, &i.1);
        }
        //win check
        if *guess == *awnser {
            break;
        }
    }
    println!("The awnser was {}", awnser);
    Ok(())
}
