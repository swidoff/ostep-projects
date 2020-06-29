use std::env;
use std::fs::File;
use std::io::{Cursor, Read};
use std::iter;
use std::process::exit;

use byteorder::{LittleEndian, ReadBytesExt};
use itertools::Itertools;

/// * Correct invocation should pass one or more files via the command line to the
///   program; if no files are specified, the program should exit with return code
///   1 and print "wzip: file1 [file2 ...]" (followed by a newline) or
///   "wunzip: file1 [file2 ...]" (followed by a newline) for **wzip** and
///   **wunzip** respectively.
/// * The format of the compressed file must match the description above exactly
///   (a 4-byte integer followed by a character for each run).
/// * Do note that if multiple files are passed to **wzip*, they are compressed
///   into a single compressed output, and when unzipped, will turn into a single
///   uncompressed stream of text (thus, the information that multiple files were
///   originally input into **wzip** is lost). The same thing holds for
///   **wunzip**.

fn main() {
    let file_names = env::args().skip(1).collect_vec();

    if file_names.is_empty() {
        println!("wunzip: file1 [file2 ...]");
        exit(1)
    }

    let chunks = file_iter(&file_names)
        .flat_map(|read| read.bytes())
        .into_iter()
        .chunks(5);

    for chunk in &chunks {
        let chunk_bytes: Vec<u8> = chunk
            .take(5)
            .map(|b| b.expect("Failed to read byte"))
            .collect_vec();
        let size = Cursor::new(&chunk_bytes[0..5]).read_i32::<LittleEndian>().unwrap();
        let c = chunk_bytes[4] as char;
        iter::repeat(c).take(size as usize).for_each(|_| print!("{}", c));
    }

    exit(0);
}


fn file_iter<'a>(file_names: &'a Vec<String>) -> impl Iterator<Item=impl Read> + 'a {
    file_names.iter().map(|file_name| {
        match File::open(file_name) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("wunzip: cannot open file");
                exit(1);
            }
        }
    })
}

