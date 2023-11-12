#![warn(clippy::all, clippy::pedantic)]
mod editor;

use editor::Editor;

fn main() {
    Editor::default().run();
}

/*

use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}


fn die(e: std::io::Error) {
    panic!("{}",e.to_string()); //macro that crashses program with an error message
    // rust does not allow try catch blocks
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    for key in io::stdin().keys() {            
            match key {            
                Ok(key) => match key {            
                    Key::Char(c) => {            
                        if c.is_control() {            
                            println!("{:?}\r", c as u8);            
                        } else {            
                            println!("{:?} ({})\r", c as u8, c);            
                        }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(err) => die(err),
        }
    }
    
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == 'q' as u8 {
                    break;
                }
            }
            Err(err) => die(err),
        }
        
        let b = b.unwrap(); 
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        if c == 'q' {
            break;
        }
        */
