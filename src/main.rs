extern crate regex;

use std::env;
use std::io::{self, Read, Write};

use regex::{Captures, Regex};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("Error reading from standard input");

    let regex = Regex::new("\\$[\\w_]+").unwrap();

    let replaced = regex.replace_all(&text, |captures: &Captures| {
        let varname = &captures[0];
        env::var(&varname[1..]).unwrap_or_else(|_| String::from(varname))
    });

    io::stdout()
        .write_all(replaced.as_bytes())
        .expect("Error writing to standard output");
}
