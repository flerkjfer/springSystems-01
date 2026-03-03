/*Tasks

Complete the save_books() function to write all books to a file. Each book should be on a separate line with fields separated by commas.
Implement the load_books() function to read books from the file and return a Vec<Book>.
Run the program and verify that it correctly saves and loads the books.

*/


use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

// Function to save books to a file
fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    // Open or create the file. Overwrites if it already exists.
    let mut file = File::create(filename).expect("Failed to create file"); 
    for bookvar in books {
        writeln!(file, "{},{},{}", bookvar.title, bookvar.author, bookvar.year) //writeln adds new line automatically
        .expect("Failed to write"); 
    }

}

// Function to load books from a file
fn load_books(filename: &str) -> Vec<Book> {
    // Open the file for reading
    let myfile = File::open(filename).expect("Failed to open file");

    // Wrap it in a buffered reader for efficient line by line reading
    let myreader = BufReader::new(myfile);

    let mut mybooks = Vec::new(); // Create an empty vector to hold the books we read

    //for each line 
    for myline in myreader.lines() {
        // Unwrap the Result to get the line string
        let myline = myline.expect("Failed to read line");

        // Split the line by commas into slices: [title, author, year]
        let myslices: Vec<&str> = myline.split(',').collect();

        if myslices.len() == 3 { // Ensure the line has exactly 3 parts before trying to parse
            let mytitle = myslices[0].to_string(); //creates owned strings
            let myauthor = myslices[1].to_string();
            let myyear: u16 = myslices[2].parse().expect("Invalid year");

            // Create a Book struct and push it into our vector
            mybooks.push(Book { title: mytitle, author: myauthor, year: myyear });
        }
    }

    mybooks
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");

    //print each loaded book 
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}