use colored::*;
use std::cmp::PartialEq;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
//the diffrent colors a letter in the guess can be. Anything that isn't a match can be assumed as white.
#[derive(Debug, PartialEq)]
pub enum Color {
    Yellow,
    Green,
}
//exracts the words from a file as a Vec
pub fn get_words(path: String) -> io::Result<Vec<String>> {
    //*exracts the words
    let file = File::open(Path::new(&path))?;
    let reader = BufReader::new(&file);
    //gets the contents as a Vec
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

pub fn valid_guess(guess: &String) -> io::Result<bool> {
    //fast check
    if guess.len() != 5 {
        return Ok(false);
    }
    //*loops through the valid words
    let words = get_words("valid_words.txt".to_string())?;
    for word in words {
        if &word == guess {
            return Ok(true);
        }
    }
    return Ok(false);
}

pub fn print_guess(
    guess: &String,
    matches: &Vec<(u8, Color)>,
    available_letters: &mut Vec<(char, (i8, bool))>,
) {
    //the letters that were found from the current guess
    let mut current_letters = vec![
        ('a', (0, false)),
        ('b', (0, false)),
        ('c', (0, false)),
        ('d', (0, false)),
        ('e', (0, false)),
        ('f', (0, false)),
        ('g', (0, false)),
        ('h', (0, false)),
        ('i', (0, false)),
        ('k', (0, false)),
        ('j', (0, false)),
        ('l', (0, false)),
        ('m', (0, false)),
        ('n', (0, false)),
        ('o', (0, false)),
        ('p', (0, false)),
        ('q', (0, false)),
        ('r', (0, false)),
        ('s', (0, false)),
        ('t', (0, false)),
        ('u', (0, false)),
        ('v', (0, false)),
        ('w', (0, false)),
        ('x', (0, false)),
        ('y', (0, false)),
        ('z', (0, false)),
    ];
    //loops through the letter of the user's guess
    for i in 0..guess.len() {
        //this flag dictate wether or not a match was found
        let mut found = false;
        //the current letter
        let letter = String::from(guess.clone().chars().nth(i).unwrap());
        //the position of the letter in teh available letters. This is found by assuming the Vec to be alphabetical
        let letter_position = (guess.clone().chars().nth(i).unwrap() as u8 - 'a' as u8) as usize;
        //look for matches
        for j in matches {
            //prints the match with the correct colour
            if j.0 == i as u8 {
                current_letters[letter_position].1 .0 += 1;
                //makes sure a match for the letter was not already found
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
            let letter = guess.clone().chars().nth(i).unwrap();
            //removes the letter from the available letters as it is not in the answer
            current_letters[letter_position].1 .1 = true;
            print!("{}", letter);
            io::stdout().flush().unwrap();
        }
        io::stdout().flush().unwrap();
    }
    //*updates the available letters
    for i in 0..current_letters.len() {
        //only updates the value if the letter times is not absolute and more matches were found
        if !available_letters[i].1 .1
            && (available_letters[i].1 .0 < current_letters[i].1 .0
                || current_letters[i].1 .1 == true)
        {
            available_letters[i] = current_letters[i];
        }
    }
    println!("")
}
pub fn check_guess(answer: &String, word: &String) -> Vec<(u8, Color)> {
    let mut matches: Vec<(u8, Color)> = Vec::new();
    //the guess and answer but with the matches replaced with placeholders
    let mut index_guess = word.clone();
    let mut index_answer = answer.clone();
    //*searches for green matches
    //we use a loop so that i can be decramented
    for i in 0..index_answer.len() {
        if index_guess.chars().nth(i) == index_answer.chars().nth(i) {
            //adds the match
            matches.push((i as u8, Color::Green));
            //replaces the match with placeholders
            index_answer.remove(i);
            index_answer.insert(i as usize, '_');
            index_guess.remove(i);
            index_guess.insert(i as usize, '_');
        }
    }
    //checks if the word was guessed
    if matches.len() == answer.len() {
        return matches;
    }
    //*searches for yellow matches
    //resets the offset for the yellows
    for i in 0..index_guess.len() {
        //the current letter that was guessed
        let found = index_guess.chars().nth(i as usize).unwrap();
        //skips the placeholders
        if found == '_' {
            continue;
        }
        if index_answer.contains(found) {
            //adds the match
            matches.push((i as u8, Color::Yellow));
            index_guess.remove(i);
            index_guess.insert(i, '_');
            //searches for the letter and replaces it with _
            let index = index_answer.chars().position(|c| c == found).unwrap();
            index_answer.remove(index);
            index_answer.insert(index, '_');
        }
    }
    return matches;
}
