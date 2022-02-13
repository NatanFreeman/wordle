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
fn valid_guess(guess: &String) -> io::Result<bool> {
    //fast check
    if guess.len() != 5 {
        return Ok(false);
    }
    //*exracts the words
    let file = File::open(Path::new("valid_words.txt"))?;
    let reader = BufReader::new(&file);
    //gets the contents as a Vec
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    //*loops through the valid words
    for word in lines {
        if &word == guess {
            return Ok(true);
        }
    }
    return Ok(false);
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
    //the guess and awnser but with the matches replaced with placeholders
    let mut index_guess = word.clone();
    let mut index_awnser = awnser.clone();
    //*searches for green matches
    //we use a loop so that i can be decramented
    for i in 0..index_awnser.len() {
        if index_guess.chars().nth(i) == index_awnser.chars().nth(i) {
            //adds the match
            matches.push((i as u8, Color::Green));
            //replaces the match with placeholders
            index_awnser.remove(i);
            index_awnser.insert(i as usize, '_');
            index_guess.remove(i);
            index_guess.insert(i as usize, '_');
        }
    }
    //checks if the word was guessed
    if matches.len() == awnser.len() {
        return matches;
    }
    //*searched for yellow matches
    //resets the offset for the yellows
    for i in 0..index_guess.len() {
        //the current letter that was guessed
        let found = index_guess.chars().nth(i as usize).unwrap();
        //skips the placeholders
        if found=='_'{
            continue;
        }
        if index_awnser.contains(found) {
            //the position in the awnser that correlates to the found
            let found_position = index_awnser.chars().position(|c| c == found).unwrap();
            //adds the match
            matches.push((i as u8, Color::Yellow));
            index_guess.remove(i);
            index_guess.insert(i, '_');
            //searches for the letter and replaces it with _
            let index = index_awnser.chars().position(|c| c == found).unwrap();
            index_awnser.remove(index);
            index_awnser.insert(index, '_');
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
    let mut i = 0;
    //we use a loop so that we can conditionolize the index
    loop {
        if i >= 6 {
            break;
        }
        //*finds matches
        //gets the guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        //ensures the string is five letters long
        guess.truncate(5);
        //clears the screen
        print!("\x1B[2J\x1B[1;1H");
        //checks if the word is a valid guess
        let valid = valid_guess(&guess)?;
        if valid {
            i += 1;
            //adds the guess to the Vec
            guesses.push((guess.clone(), check_guess(awnser, &guess)));
        }
        //prints the guesses
        for i in &guesses {
            print_guess(&i.0, &i.1);
        }
        if !valid {
            println!("Not in word list");
        }
        //win check
        if *guess == *awnser {
            break;
        }
    }
    println!("The awnser was {}", awnser);
    Ok(())
}
