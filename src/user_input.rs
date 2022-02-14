use std::io;

/// Ask the user for a phrase and password length.
pub fn prompt_for_user_input() -> (String, usize) {
    println!("Enter a sentence or sequence of words with capitalization and punctuation:");
    let mut phrase = String::new();
    io::stdin().read_line(&mut phrase).unwrap();

    let mut pass_len: Option<usize> = None;
    println!("How long do you want your password to be?");
    while let None = pass_len {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read the user's input
        let input = input.trim(); // Remove whitespace
        
        match input.parse::<usize>() { // Parse the String into an integer
            Ok(n) => pass_len = Some(n),
            Err(_) => println!("Please enter a positive integer") // If the string cannot be parsed
        };
    }
    (phrase, pass_len.unwrap())
}