#![cfg(test)]
use super::*;
use crate::print;
use crate::println;

#[test_case]
fn println_simple() {
    print!("output");
    println!("output");
}

#[test_case]
fn println_overflow() {
    for _ in 0..200 {
        println!("output");
    }
}

#[test_case]
fn println_output() {
    let s = "Single line of text.";
    println!("{}", s);
    let writes = writer();
    let row = buffer::Buffer::last_row() - 1;
    for (i, b) in s.bytes().enumerate() {
        let byte = writes.read_byte(row, i);
        assert_eq!(b as char, byte as char);
    }
}
