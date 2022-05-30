/* Lexer & Parser*/

use std::fs;



//Take .piss file and generate parsable data.
pub fn lex_file(filepath: &str) -> Vec<String> {
    //Convert file to String and create an empty Vector for storing sentences.
    let raw_data = fs::read_to_string(filepath).expect("Error Reading File.");
    let mut final_data = Vec::new();

    //For each line, if it isn't a comment or empty, add it to the final data Vector.
    for line in raw_data.lines() {
        if !line.starts_with("~") && !line.trim().is_empty() {final_data.push(line.trim().to_string());}
    }

    final_data
}


//Parses sentences
pub fn parse_sentence(sentence: &String) {
    for word in sentence.split_whitespace().into_iter() {
        println!("{}", word)
    }
}