extern crate termion;

use termion::{TermWrite, IntoRawMode, Color, Style};
use std::io::{Read, Write, stdout, stdin};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    stdout.goto(5, 5).unwrap();
    stdout.clear().unwrap();
    stdout.style(Style::Bold).unwrap();
    stdout.write(b"yo, 'q' will exit.").unwrap();
    stdout.reset().unwrap();
    stdout.flush().unwrap();
    stdout.goto(20, 10).unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            b'q' => return,
            b'c' => stdout.clear(),
            b'r' => stdout.color(Color::Rgb(5, 0, 0)),
            a => stdout.write(&[a]),
        }.unwrap();

        stdout.flush().unwrap();
    }
}
