/*A wordle program*/
use rand::{thread_rng, Rng};
use std::fs;
use std::io::Write;
use std::fs::File;
use colored::*;
use std::cmp::PartialEq;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
//the diffrent colors a letter in the guess can be. Anything that isn't a match can be assumed as white.
#[derive(Debug, PartialEq)]
enum Color {
    Yellow,
    Green,
}
fn guess(awnser: &String, word: &String) -> Vec<(u8, Color)> {
    let mut matches: Vec<(u8, Color)> = Vec::new();
    //we copy the original guess and awnser and remove the matches from this value so that we can search for yellows
    let mut trimmed_guess = word.clone();
    let mut trimmed_awnser = awnser.clone();
    //*searches for green matches
    //we use a loop so that i can be decramented
    let mut i = 0;
    //becuse we mutate the
    let mut offset = 0;
    loop {
        //checks if the word is finished
        if i >= trimmed_awnser.len() {
            break;
        }
        if trimmed_guess.chars().nth(i) == trimmed_awnser.chars().nth(i) {
            //adds the match
            matches.push((i as u8 + offset, Color::Green));
            trimmed_guess.remove(i);
            trimmed_awnser.remove(i);
            offset += 1
        } else {
            i += 1;
        }
    }
    //checks if the word was guessed
    if matches.len()==awnser.len(){
        return matches;
    }
    //*searched for yellow matches
    for i in 0..trimmed_guess.len() - 1 {
        let found = trimmed_guess.chars().nth(i);
        match found {
            Some(letter) => {
                if trimmed_awnser.contains(letter) {
                    //adds the match
                    matches.push((i as u8, Color::Yellow));
                    trimmed_guess.remove(i);
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
    println!("{}", awnser);
    for _ in 0..5 {
        let mut bru=String::new();
        io::stdin().read_line(&mut bru)?;
        bru.pop();
        bru.pop();
        let matches=guess(awnser, &bru);
        println!("{:?}", matches);
        for i in 0..awnser.len(){
            let found=matches.get(i);
            //prints the character normally if there is no match
            if found==None{
                print!("{}", bru.clone().chars().nth(i).unwrap());
                continue;
            }
            let found=found.unwrap();
            if found.0==i as u8{
                let letter=String::from(bru.clone().chars().nth(i).unwrap());
                print!("{}", letter.green());
            }
            else{
                print!("{}", bru.clone().chars().nth(i).unwrap());
            }
            io::stdout().flush().unwrap();
        }
        println!("")
    }
    Ok(())
}
