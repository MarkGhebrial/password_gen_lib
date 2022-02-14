mod password_generator;
mod user_input;

use password_generator::*;
use user_input::*;

/// Take a multiword phrase and turn it into a memorable password based
/// on the first letters of each word in the phrase.
pub fn generate_memorable_password(phrase: &str) -> String {
    obscure_string(&generate_password(&phrase.to_string()))
}

/// Take a multiword phrase and turn it into a memorable password based
/// on the first letters of each word in the phrase.
/// 
/// This function differs from `generate_memorable_password` by producing
/// a password whoose length depends not on that of the phrase, but rather
/// on the `len` parameter
pub fn generate_memorable_password_with_len(phrase: &str, len: &usize) -> String {
    repeat_string_for_length(
        &generate_memorable_password(phrase), 
        len
    )
}

/// Generate a memorable password based off of the user's input
pub fn prompt_for_memorable_password() -> String {
    let (phrase, password_length) = prompt_for_user_input();
    generate_memorable_password_with_len(&phrase, &password_length)
}