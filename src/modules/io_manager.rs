use std::{
    io::{self, Write},
    process::Command,
};

pub fn write_console(message: &str) {
    print!("{}", message.to_string());
    io::stdout().flush();
}
pub fn write_line_console(message: &str) {
    println!("{}", message.to_string());
}

pub fn read_console() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something wasnt right");
    return remove_escape_chars(input);
}
fn remove_escape_chars(input: String) -> String {
    let escape_chars = ['\\', '\"', '\'', '\n', '\r', '\t', '\0'];
    let mut result = input;

    for c in escape_chars.iter() {
        result = result.replace(*c, "");
    }

    result
}
