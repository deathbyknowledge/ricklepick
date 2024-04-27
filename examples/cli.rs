use ricklepick::Parser;
use ricklepick::value::{Instance, Value};
use std::fs::File;
use std::io::{BufReader, Read};

// torch's constnat values
const MAGIC_NUMBER: i128 = 0x1950a86a20f9469cfc6c;
const PROTOCOL_VERSION: u32 = 1001;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let model_file = match args.as_slice() {
        [_, m] => m.to_owned(),
        _ => {
            eprintln!("usage: {} [FILEPATH]", args[0]);
            return;
        }
    };

    println!("Opening file {model_file}\n");

    let mut buf = BufReader::new(
        File::open(model_file.clone()).unwrap_or_else(|_| panic!("File not found {}", model_file)),
    );
    let mut parser = Parser::from(&mut buf);
    parser.add_extension("torch.storage", "_load_from_bytes", load_from_bytes);
    parser.add_extension("collections", "OrderedDict", ordered_dict);
    parser.add_extension("torch._utils", "_rebuild_tensor_v2", rebuild_tensor);
    let result = parser.parse();
    println!("{result}");
}


fn ordered_dict(val: Value) -> Value {
    val
}  

// def _rebuild_tensor_v2(storage, storage_offset, size, stride, requires_grad, backward_hooks, metadata=None)
fn rebuild_tensor(val: Value) -> Value {
    if let Value::Tuple(arg) = val.clone() {
        let (tensor, _offset, _size, _stride, _required_grad) = rebuild_tensor_args_from_tuple(arg);
        return Value::List(tensor);
    }
    val
}  

fn rebuild_tensor_args_from_tuple(mut tuple: Vec<Value>) -> (Vec<Value>, u32, u32, u32, bool) {
    if tuple.len() != 6 {
        panic!("Wrong arguments for _rebuild_tensor_v2")
    }
    tuple.pop();
    let requires_grad = tuple.pop().unwrap().as_bool().unwrap();
    let stride = if let Some(Value::Tuple(x)) = tuple.pop() { x[0].clone().as_uint().unwrap() } else { panic!("error parsing stride") };
    let size = if let Some(Value::Tuple(x)) = tuple.pop() { x[0].clone().as_uint().unwrap() } else { panic!("error parsing size") };
    let offset = tuple.pop().unwrap().as_uint().unwrap();
    let tensor = if let Some(Value::List(x)) = tuple.pop() { x } else { panic!("error parsing storage") };
    (tensor, offset, size, stride, requires_grad)
}


//https://github.com/pytorch/pytorch/blob/main/torch/serialization.py#L1184-L1193
fn persistence_load_args(args: Value) -> (String, Instance, String, String, u32) {
    if let Value::Tuple(mut tuple) = args {
        if tuple.len() != 6 {
            panic!("Wrong arguments for _persistence_load_args")
        }
        tuple.pop(); // don't care about view_metadata
        let numel = tuple.pop().unwrap().as_uint().unwrap();
        let location = tuple.pop().unwrap().as_string().unwrap();
        let root_key = tuple.pop().unwrap().as_string().unwrap();
        let storage_type = tuple.pop().unwrap().as_instance().unwrap();
        let typename = tuple.pop().unwrap().as_string().unwrap();
        (typename, storage_type, root_key, location, numel)
    } else {
        panic!("was not tuple")
    }
}

fn storage_size(s: String) -> usize {
    match s.as_str() {
        "LongStorage" => 8,
        _ => panic!(""),

    }
}

// https://pytorch.org/docs/stable/_modules/torch/serialization.html#load
fn load_from_bytes(val: Value) -> Value {
    if let Value::Tuple(arg) = val {
        if let Value::Bytes(bytes) = &arg[0] {
            let mut buf = bytes.as_slice();

            // torch magic number
            let mut parser = Parser::from(&mut buf);
            let magic_number = parser.parse().as_long().unwrap();
            if magic_number != MAGIC_NUMBER {
                panic!("Wrong magic number. Corrupted file?");
            }

            // torch protocol version
            let mut parser = Parser::from(&mut buf);
            let protocol_version = parser.parse().as_uint().unwrap();
            if protocol_version != PROTOCOL_VERSION {
                panic!("Wrong protocl version. Got {protocol_version}");
            }

            // encoded sys info
            let mut parser = Parser::from(&mut buf);
            let _sys_info = parser.parse();

            // 
            let mut parser = Parser::from(&mut buf);
            let args = parser.parse();
            let (_typename, storage_type, _root_key, _location, numel) = persistence_load_args(args);
            println!("_load_from_bytes:\n\tPROTOCOL VERSION: {protocol_version}\n\tSYS_INFO: {_sys_info}\n\tLOADING TENSOR OF SIZE ({numel} * {})", storage_size(storage_type.name()));
            let mut parser = Parser::from(&mut buf);
            let _keys = parser.parse();
            let mut tmp = [0; 8];
            let _ = buf.read_exact(&mut tmp);
            let to_read = u64::from_le_bytes(tmp);
            let mut tensor = vec![];
            for _ in 0..to_read {
                let _ = buf.read_exact(&mut tmp);
                let x = u64::from_le_bytes(tmp);
                tensor.push(Value::ULong(x as u128)); 
            }
            return Value::List(tensor);
        }
    } else {
        panic!("was not tuple");
    }
    panic!("w/e")
}  