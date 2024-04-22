use ricklepick::load;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let model_file = match args.as_slice() {
        [_, m] => m.to_owned(),
        _ => {
            eprintln!("usage: main model.pt");
            return;
        }
    };

    println!("Opening file {model_file}\n");

    let mut buf = BufReader::new(
        File::open(model_file.clone()).unwrap_or_else(|_| panic!("File not found {}", model_file)),
    );
    let result = load(&mut buf);
    println!("{result}");
}
