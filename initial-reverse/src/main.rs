use std::env;
use std::fs::File;
use std::io::{Bytes, Read, stdin, stdout, Write};
use std::os::linux::fs::MetadataExt;
use std::process::exit;

use itertools::Itertools;
use std::rc::Rc;

fn main() {
    let file_names = env::args().skip(1).collect_vec();
    match file_names.len() {
        0 => {
            let in_file = stdin();
            let mut out_file = stdout();
            write_lines_reversed(&mut out_file, collect_lines(in_file));
        }
        1 => {
            let in_file = open_file(&file_names[0]);
            let mut out_file = stdout();
            write_lines_reversed(&mut out_file, collect_lines(in_file));
        }
        2 => {
            let in_file = open_file(&file_names[0]);
            if is_the_same_file_as(&file_names[1], &in_file) {
                eprintln!("reverse: input and output file must differ");
                exit(1);
            }
            let mut out_file = create_file(&file_names[1]);
            write_lines_reversed(&mut out_file, collect_lines(in_file));
        }
        _ => {
            eprintln!("usage: reverse <input> <output>");
            exit(1);
        }
    };

    exit(0);
}

fn open_file(file_name: &String) -> File {
    match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("reverse: cannot open file '{}'", file_name);
            exit(1);
        }
    }
}

fn create_file(file_name: &String) -> File {
    match File::create(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("reverse: cannot open file '{}'", file_name);
            exit(1);
        }
    }
}

fn is_the_same_file_as(file_name: &String, other_file: &File) -> bool {
    fn st_ino(file: &File) -> u64 {
        file.metadata().expect("Unable to get file metadata").st_ino()
    }
    File::open(file_name)
        .map(|new_file| st_ino(&new_file) == st_ino(other_file))
        .unwrap_or(false)
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
        Some(String::from_utf8(buf).expect("Expected valid utf."))
    }
}

fn collect_lines<'a, R: Read + 'a>(read: R) -> Stack<String> {
    read.bytes()
        .batching(|b_iter| read_line(b_iter))
        .fold(Stack::new(), |stack, v| stack.push(v))
}

fn write_lines_reversed<'a, W: Write + 'a>(write: &mut W, lines: Stack<String>) {
    let mut head = lines;
    while let Some((line, new_head)) = head.pop() {
        write.write(line.as_bytes()).expect("Unable to write line.");
        head = new_head;
    }
}

enum StackElement<T> {
    Cons(T, Rc<StackElement<T>>),
    Nil,
}

struct Stack<T> {
    head: Rc<StackElement<T>>
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        return Stack { head: Rc::new(StackElement::Nil) };
    }

    fn push(&self, value: T) -> Stack<T> {
        return Stack { head: Rc::new(StackElement::Cons(value, Rc::clone(&self.head))) };
    }

    fn pop(&self) -> Option<(&T, Stack<T>)> {
        match &*self.head {
            StackElement::Cons(value, tail) => {
                Some((value, Stack { head: Rc::clone(&tail) }))
            }
            StackElement::Nil => None
        }
    }
}