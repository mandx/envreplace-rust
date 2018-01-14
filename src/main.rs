extern crate regex;

use std::env;
use std::io::{self, Read, Write};

use regex::{Regex, Captures};


fn main() {
    let mut text = String::new();
    let read_result = io::stdin().read_to_string(&mut text);

    if let Err(error) = read_result {
        eprintln!("Error reading from stdin: {}", error);
        return
    }

    let regex = Regex::new("\\$[\\w_]+").unwrap();

    let replaced = regex.replace_all(&text, |captures: &Captures| {
        let varname = &captures[0];
        match env::var(&varname[1..]) {
            Ok(value) => value,
            _ => String::from(varname),
        }
    });

    let write_result = io::stdout().write(replaced.as_bytes());
    if let Err(error) = write_result {
        eprintln!("Error writing to stdout: {}", error);
    }
}
