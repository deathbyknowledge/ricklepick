// https://github.com/python/cpython/blob/3.12/Lib/pickletools.py
use std::fs::File;
use std::io::{BufRead, BufReader};

mod op;
mod value;
mod vm;
use value::Value;
use vm::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let model_file = match args.as_slice() {
        [_, m] => m.to_owned(),
        _ => {
            eprintln!("usage: main model.pt");
            return;
        }
    };

    println!("Opening file {model_file}");

    let mut buf = BufReader::new(
        File::open(model_file.clone()).expect(format!("File not found {}", model_file).as_str()),
    );
    let result = parse(&mut buf);
    println!("Pickle contained => {:?}", result);
}

fn parse(buf: &mut dyn BufRead) -> Value {
    let mut vm = VM::from(buf);

    loop {
        if !vm.step() {
            return vm.result().expect("did not expect an error tbh");
        }
    }

}

fn dump(buff: &[u8]) {
    let mut as_chars: [char; 16] = ['.'; 16];
    let count = buff.len();
    for (idx, byte) in buff.iter().enumerate() {
        // Line count in hex at the every 16 bytes
        if idx != 0 && idx % 16 == 0 {
            print!("  {}", as_chars.iter().cloned().collect::<String>());
        }
        if idx % 16 == 0 {
            print!("\n{idx:>08x}: ");
        }

        // End of line buffer with bytes as ascii characters;
        as_chars[idx % 16] = byte_to_char(*byte);

        // Print byte
        print!("{:>02x}{}", byte, if idx % 2 == 0 { "" } else { " " });

        if idx + 1 == count {
            for _ in 0..((count - idx + 1) % 16) {
                print!("  ");
            }
            print!("  {}", as_chars.iter().cloned().collect::<String>());
        }
    }
    println!("");
}

fn byte_to_char(byte: u8) -> char {
    if byte >= ' ' as u8 && byte <= '~' as u8 {
        return byte as char;
    } else {
        return '.';
    }
}
