use std::env;
use std::fs::File;
use std::io::{Bytes, Read};

use itertools::{Itertools, Batching};
use std::process::exit;

///**Details**
///
/// * Your program **wcat** can be invoked with one or more files on the command
///   line; it should just print out each file in turn.
/// * In all non-error cases, **wcat** should exit with status code 0, usually by
///   returning a 0 from **main()** (or by calling **exit(0)**).
/// * If *no files* are specified on the command line, **wcat** should just exit
///   and return 0. Note that this is slightly different than the behavior of
///   normal UNIX **cat** (if you'd like to, figure out the difference).
/// * If the program tries to **fopen()** a file and fails, it should print the
///   exact message "wcat: cannot open file" (followed by a newline) and exit
///   with status code 1.  If multiple files are specified on the command line,
///   the files should be printed out in order until the end of the file list is
///   reached or an error opening a file is reached (at which point the error
///   message is printed and **wcat** exits).
fn main() {
    env::args()
        .skip(1)
        .map(|filename| File::open(filename))
        .for_each(|file_result| {
            match file_result {
                Ok(file) => {
                    line_iter(file).for_each(|s| println!("{}", s))
                }
                Err(_) => {
                    println!("wcat: cannot open file");
                    exit(1);
                }
            };
        });

    exit(0);
}

fn line_iter(file: File) -> Batching<Bytes<File>, fn(&mut Bytes<File>) -> Option<String>> {
    file.bytes().batching(|b_iter| {
        let buf = b_iter
            .map(|b| b.unwrap())
            .take_while(|b| *b != 0x0A as u8)
            .collect_vec();
        if buf.is_empty() {
            None
        } else {
            Some(String::from_utf8(buf).expect("Expected valid utf"))
        }
    })
}
