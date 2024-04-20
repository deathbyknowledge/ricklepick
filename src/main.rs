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
    println!("{result}");
}

fn parse(buf: &mut dyn BufRead) -> Value {
    let mut vm = VM::from(buf);

    loop {
        if !vm.step() {
            return vm.result().expect("did not expect an error tbh");
        }
    }

}