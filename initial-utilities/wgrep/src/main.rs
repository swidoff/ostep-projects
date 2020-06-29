#![feature(iter_map_while)]

use std::env;
use std::fs::File;
use std::io::{Read, stdin, Bytes};
use std::process::exit;

use itertools::Itertools;

/// * Your program **wgrep** is always passed a search term and zero or
///   more files to grep through (thus, more than one is possible). It should go
///   through each line and see if the search term is in it; if so, the line
///   should be printed, and if not, the line should be skipped.
/// * The matching is case sensitive. Thus, if searching for **foo**, lines
///   with **Foo** will *not* match.
/// * Lines can be arbitrarily long (that is, you may see many many characters
///   before you encounter a newline character, \\n). **wgrep** should work
///   as expected even with very long lines. For this, you might want to look
///   into the **getline()** library call (instead of **fgets()**), or roll your
///   own.
/// * If **wgrep** is passed no command-line arguments, it should print
///   "wgrep: searchterm [file ...]" (followed by a newline) and exit with
///   status 1.
/// * If **wgrep** encounters a file that it cannot open, it should print
///   "wgrep: cannot open file" (followed by a newline) and exit with status 1.
/// * In all other cases, **wgrep** should exit with return code 0.
/// * If a search term, but no file, is specified, **wgrep** should work,
///   but instead of reading from a file, **wgrep** should read from
///   *standard input*. Doing so is easy, because the file stream **stdin**
///   is already open; you can use **fgets()** (or similar routines) to
///   read from it.
/// * For simplicity, if passed the empty string as a search string, **wgrep**
///   can either match NO lines or match ALL lines, both are acceptable.
fn main() {
    let search_term = match env::args().skip(1).next() {
        Some(s) => s,
        None => {
            println!("wgrep: searchterm [file ...]");
            exit(1);
        }
    };

    let file_names = env::args().skip(2).collect_vec();

    if file_names.is_empty() {
        grep(stdin(), &search_term)
            .for_each(|line| print!("{}", line));
    } else {
        file_iter(&file_names)
            .flat_map(|file| grep(file, &search_term))
            .for_each(|line| print!("{}", line));
    };


    exit(0);
}

fn file_iter<'a>(file_names: &'a Vec<String>) -> impl Iterator<Item=impl Read> + 'a {
    file_names.iter().map(|file_name| {
        match File::open(file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("wgrep: cannot open file");
                exit(1);
            }
        }
    })
}

fn grep<'a, R: Read + 'a>(read: R, search_term: &'a String) -> impl Iterator<Item=String> + 'a {
    read.bytes()
        .batching(|b_iter| read_line(b_iter))
        .filter(move |str| str.contains(search_term))
}

fn read_line<R: Read>(b_iter: &mut Bytes<R>) -> Option<String> {
    let mut hit_eol = false;
    let mut buf: Vec<u8> = Vec::new();
    while let Some(Ok(c)) = b_iter.next() {
        buf.push(c);
        if c == 0x0A as u8 {
            hit_eol = true;
            break;
        }
    }
    if buf.is_empty() && !hit_eol {
        None
    } else {
        Some(String::from_utf8(buf).expect("Expected valid utf"))
    }
}
