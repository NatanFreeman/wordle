/*A wordle program*/
use rand::Rng;
use std::io;
use webster;
mod wordle;
//returns all the possible answers based off the guesses
fn possible_answers(
    guesses: &Vec<(String, Vec<(u8, wordle::Color)>)>,
    available_letters: &Vec<(char, (i8, bool))>,
) -> io::Result<Vec<String>> {
    let answers = wordle::get_words("valid_words.txt".to_string())?;
    //the possible words that have been found
    let mut possible_found: Vec<String> = Vec::new();
    if guesses.len() == 0 {
        return Ok(possible_found);
    }
    //checks a word in the accepted words file
    for answer in answers {
        //this flag says wether or not the answer is the possible solution
        let mut possible = true;
        //makes sure the letters in the answer are available
        for i in answer.chars() {
            //the position of the letter in teh available letters. This is found by assuming the Vec to be alphabetical
            let letter_position = (i as u8 - 'a' as u8) as usize;
            //checks if the number of matches may be bigger than the number listed
            if available_letters[letter_position].1 .1 == true {
                if answer.matches(i).count() != available_letters[letter_position].1 .0 as usize {
                    possible = false;
                    break;
                }
            } else {
                if answer.matches(i).count() < available_letters[letter_position].1 .0 as usize {
                    possible = false;
                    break;
                }
            }
            if !possible {
                continue;
            }
            for guess in guesses {
                //makes sure that the possible answer wasn't guessed
                if guess.0 == answer {
                    possible = false;
                    break;
                }
                //matches the patterns
                for i in &guess.1 {
                    if i.1 == wordle::Color::Green {
                        if answer.chars().nth(i.0 as usize).unwrap()
                            != guess.0.chars().nth(i.0 as usize).unwrap()
                        {
                            possible = false;
                        }
                    } else if i.1 == wordle::Color::Yellow {
                        if !answer.contains(guess.0.chars().nth(i.0 as usize).unwrap()) {
                            possible = false;
                        }
                        //makes sure the letter is in the wrong place
                        else {
                            if answer.chars().nth(i.0 as usize).unwrap()
                                == guess.0.chars().nth(i.0 as usize).unwrap()
                            {
                                possible = false;
                            }
                        }
                    }
                }
            }
        }
        if !possible {
            continue;
        }
        possible_found.push(answer.clone());
    }
    Ok(possible_found)
}
fn main() -> io::Result<()> {
    let words = wordle::get_words("words.txt".to_string())?;
    //*chooses a word
    let mut rng = rand::thread_rng();
    let number: usize = rng.gen_range(0..words.len());
    //the answer the player will have to guess
    let answer = words.get(number).unwrap();
    println!("{}", answer);
    //the guesses the user has said so far
    let mut guesses: Vec<(String, Vec<(u8, wordle::Color)>)> = Vec::new();
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
        guess = guess.to_lowercase();
        //clears the screen
        print!("\x1B[2J\x1B[1;1H");
        //checks if the word is a valid guess
        let valid = wordle::valid_guess(&guess)?;
        if valid {
            i += 1;
            //adds the guess to the Vec
            guesses.push((guess.clone(), wordle::check_guess(answer, &guess)));
        }
        /*the available letters.
        available_letters.0 is the letter.
        available_letters.1.0 is the number of times it can exist
        available_letters.1.1 is wether or not there can be more of the letter*/
        let mut available_letters = vec![
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
        //prints the guesses
        for i in &guesses {
            wordle::print_guess(&i.0, &i.1, &mut available_letters);
        }
        //win check
        if *guess == *answer {
            break;
        }
        if !valid {
            println!("Not in word list");
        }
        //prints the possible answers
        let answers = possible_answers(&guesses, &available_letters)?;
        if answers.len() > 0 && valid {
            print!("Possible answers: ");
            for i in answers {
                print!("{} ", i);
            }
            println!("");
        }
    }
    let definition = webster::dictionary(answer);
    match definition {
        Some(message) => println!("The answer was {}\ndefinition: {}", answer, message),
        None => println!("The answer was {}", answer),
    };
    Ok(())
}
