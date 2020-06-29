use std::env;
use itertools::Itertools;
use std::process::exit;
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::io;

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

fn main() -> io::Result<()> {
    let file_names = env::args().skip(1).collect_vec();

    if file_names.is_empty() {
        println!("wzip: file1 [file2 ...]");
        exit(1)
    }

    let groups = file_iter(&file_names)
        .flat_map(|read| read.bytes())
        .map(|result| result.expect("Failed to read file"))
        .group_by(|char| *char);

    let mut out = stdout();

    for (key, group) in &groups {
        let run_size: i32 = group.count() as i32;
        out.write_all(&run_size.to_le_bytes())?;
        out.write_all(&key.to_le_bytes())?;
    }
    out.flush();

    exit(0);
}

fn file_iter<'a>(file_names: &'a Vec<String>) -> impl Iterator<Item=impl Read> + 'a {
    file_names.iter().map(|file_name| {
        match File::open(file_name) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("wzip: cannot open file");
                exit(1);
            }
        }
    })
}