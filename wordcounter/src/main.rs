/* Create a program that:

Takes a string of text as input
Splits the text into words (space as separator) // text.split_whitespace().collect();
Counts the frequency of each word
Returns the word with the highest frequency and its count
Requirements:
Use mutable references where appropriate
Avoid using HashMaps or complex data structures
*/

use std::io; //for getting user input; library

//fn funcName(varName: varType) -> returnType
//returns a tuple: (String, usize)
fn max_freq(user_input: &str) -> (String, usize) {

    //creates borrowed (&) string slices
    let words: Vec<&str> = user_input.split_whitespace().collect();

    //make an empty mutable string 
    let mut max_word = String::new();
    let mut max_count = 0;

    for wordx in &words { //for each word in borrowed (&) string pieces
        let mut count = 0;

        for w in &words { //double loop
            if wordx == w {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = wordx.to_string(); //the assignment is now owned by the left
        }
    }

    (max_word, max_count) //return tuple
}

fn main() {
    println!("Hello, please enter some words: ");

    let mut user_input = String::new(); //make an empty mutable string

    //can be written as: io::stdin().read_line(&mut guess).expect("Failed to read line");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let (word, count) = max_freq(&user_input);

    println!("Most frequent word: \"{}\" ({} times)", word, count);
}