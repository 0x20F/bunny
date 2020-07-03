extern crate percent_encoding;


use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};


// What characters to encode
const CHARACTERS: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'/');


pub fn encode(string: &str) -> String {
    utf8_percent_encode(string, CHARACTERS).to_string()
}