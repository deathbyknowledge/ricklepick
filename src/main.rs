use std::fs::File;
use std::io::{BufReader, Read};

mod opcode;
use opcode::*;

//struct OpCode;

/// Integer or boolean, encoded with the ASCII characters [0-9-].
/// The values '00' and '01' encode the Python values `False` and `True`.
/// Normally a value would not contain leading '0' characters.
struct DecimalNlShort;

/// Integer, encoded with the ASCII chracters [0-9-], followed by 'L'.
struct DecimalNlLong;

type S4 = i32;
type S2 = i16;
type u1 = u8;
type u2 = u16;
type u4 = u32;
type long1 = i64;
type long4 = i64;
struct stringnl;
struct string1;
struct string4;

#[derive(Debug)]
enum Op {
    Int,            //(DecimalNlShort),
    BinInt,         //(S4),
    BinInt1,        //(u1),
    BinInt2,        //2(u2),
    Long,           //(DecimalNlLong),
    Long1,          //(long1),
    Long4,          //(long4),
    String,         //(stringnl),
    Binstring,      //(string4),
    ShortBinstring, //(string1)
    Binbytes,
    ShortBinbytes,
    Binbytes8,
    None,
    Newtrue,
    Newfalse,
    Unicode,
    ShortBinunicde,
    Binunicode,
    Binunicode8,
    Float,
    Binfloat,
    EmptyList,
    Append,
    Appends,
    List,
    EmptyTuple,
    Tuple,
    Tuple1,
    Tuple2,
    Tuple3,
    EmptyDict,
    Dict,
    SetItem,
    SetItems,
    EmptySet,
    AddItems,
    FrozenSet,
    Pop,
    Dup,
    Mark,
    PopMark,
    Get,
    BinGet,
    LongBinGet,
    Put,
    BinPut,
    LongBinPut,
    Memoize,
    Ext1,
    Ext2,
    Ext4,
    GlobalOpcode,
    StackGlobal,
    Reduce,
    Build,
    Inst,
    Obj,
    NewObj,
    NewObjEx,
    Proto,
    Stop,
    Frame,
    Persid,
    BinPersid,
    ByteArray8,
    NextBuffer,
    ReadonlyBuffer,
}

impl Op {
    fn read(byte: u8) -> Self {
        match byte {
            INT => Op::Int,                        //(DecimalNlShort),
            BININT => Op::BinInt,                  //(S4),
            BININT1 => Op::BinInt1,                //(u1),
            BININT2 => Op::BinInt2,                //2(u2),
            LONG => Op::Long,                      //(DecimalNlLong),
            LONG1 => Op::Long1,                    //(long1),
            LONG4 => Op::Long4,                    //(long4),
            STRING => Op::String,                  //(stringnl),
            BINSTRING => Op::Binstring,            //(string4),
            SHORT_BINSTRING => Op::ShortBinstring, //(string1)
            BINBYTES => Op::Binbytes,
            SHORT_BINBYTES => Op::ShortBinbytes,
            BINBYTES8 => Op::Binbytes8,
            NONE => Op::None,
            NEWTRUE => Op::Newtrue,
            NEWFALSE => Op::Newfalse,
            UNICODE => Op::Unicode,
            SHORT_BINUNICODE => Op::ShortBinunicde,
            BINUNICODE => Op::Binunicode,
            BINUNICODE8 => Op::Binunicode8,
            FLOAT => Op::Float,
            BINFLOAT => Op::Binfloat,
            EMPTY_LIST => Op::EmptyList,
            APPEND => Op::Append,
            APPENDS => Op::Appends,
            LIST => Op::List,
            EMPTY_TUPLE => Op::EmptyTuple,
            TUPLE => Op::Tuple,
            TUPLE1 => Op::Tuple1,
            TUPLE2 => Op::Tuple2,
            TUPLE3 => Op::Tuple3,
            EMPTY_DICT => Op::EmptyDict,
            DICT => Op::Dict,
            SET_ITEM => Op::SetItem,
            SET_ITEMS => Op::SetItems,
            EMPTY_SET => Op::EmptySet,
            ADD_ITEMS => Op::AddItems,
            FROZEN_SET => Op::FrozenSet,
            POP => Op::Pop,
            DUP => Op::Dup,
            MARK => Op::Mark,
            POP_MARK => Op::PopMark,
            GET => Op::Get,
            BINGET => Op::BinGet,
            LONG_BINGET => Op::LongBinGet,
            PUT => Op::Put,
            BINPUT => Op::BinPut,
            LONG_BINPUT => Op::LongBinPut,
            MEMOIZE => Op::Memoize,
            EXT1 => Op::Ext1,
            EXT2 => Op::Ext2,
            EXT4 => Op::Ext4,
            GLOBAL_OPCODE => Op::GlobalOpcode,
            STACK_GLOBAL => Op::StackGlobal,
            REDUCE => Op::Reduce,
            BUILD => Op::Build,
            INST => Op::Inst,
            OBJ => Op::Obj,
            NEW_OBJ => Op::NewObj,
            NEW_OBJ_EX => Op::NewObjEx,
            PROTO => Op::Proto,
            STOP => Op::Stop,
            FRAME => Op::Frame,
            PERSID => Op::Persid,
            BINPERSID => Op::BinPersid,
            BYTEARRAY8 => Op::ByteArray8,
            NEXT_BUFFER => Op::NextBuffer,
            READONLY_BUFFER => Op::ReadonlyBuffer,
            0..=u8::MAX => panic!("Can't parse unknown opcode {}", byte),
        }
    }
}



fn main() {
    let args: Vec<_> = std::env::args().collect();
    let model_file = match args.as_slice() {
        [_, m] => m.to_owned(),
        _ => {
            eprintln!("usage: main model.pt");
            return;
        }
    };

    let mut _buf = BufReader::new(
        File::open(model_file.clone()).expect(format!("File not found {}", model_file).as_str()),
    );
    let mut buf = vec![];
    _buf.read_to_end(&mut buf).unwrap();
    parse(&buf);
    println!("Opening file {model_file}");
}

struct VM {
  // Protocol version;
  version: u8,
}

fn parse(buf: &[u8]) {
  let mut vm = VM{version: 0};

  let mut offset = 0;
  loop {
    let opcode = Op::read(buf[offset]);
    println!("  OFFSET: {} -> OP {:?}", offset, opcode);
    match opcode {
        Op::Int => todo!(),
        Op::BinInt => todo!(),
        Op::BinInt1 => todo!(),
        Op::BinInt2 => todo!(),
        Op::Long => todo!(),
        Op::Long1 => todo!(),
        Op::Long4 => todo!(),
        Op::String => todo!(),
        Op::Binstring => todo!(),
        Op::ShortBinstring => todo!(),
        Op::Binbytes => todo!(),
        Op::ShortBinbytes => todo!(),
        Op::Binbytes8 => todo!(),
        Op::None => todo!(),
        Op::Newtrue => todo!(),
        Op::Newfalse => todo!(),
        Op::Unicode => todo!(),
        Op::ShortBinunicde => todo!(),
        Op::Binunicode => todo!(),
        Op::Binunicode8 => todo!(),
        Op::Float => todo!(),
        Op::Binfloat => todo!(),
        Op::EmptyList => todo!(),
        Op::Append => todo!(),
        Op::Appends => todo!(),
        Op::List => todo!(),
        Op::EmptyTuple => todo!(),
        Op::Tuple => todo!(),
        Op::Tuple1 => todo!(),
        Op::Tuple2 => todo!(),
        Op::Tuple3 => todo!(),
        Op::EmptyDict => todo!(),
        Op::Dict => todo!(),
        Op::SetItem => todo!(),
        Op::SetItems => todo!(),
        Op::EmptySet => todo!(),
        Op::AddItems => todo!(),
        Op::FrozenSet => todo!(),
        Op::Pop => todo!(),
        Op::Dup => todo!(),
        Op::Mark => todo!(),
        Op::PopMark => todo!(),
        Op::Get => todo!(),
        Op::BinGet => todo!(),
        Op::LongBinGet => todo!(),
        Op::Put => todo!(),
        Op::BinPut => todo!(),
        Op::LongBinPut => todo!(),
        Op::Memoize => todo!(),
        Op::Ext1 => todo!(),
        Op::Ext2 => todo!(),
        Op::Ext4 => todo!(),
        Op::GlobalOpcode => todo!(),
        Op::StackGlobal => todo!(),
        Op::Reduce => todo!(),
        Op::Build => todo!(),
        Op::Inst => todo!(),
        Op::Obj => todo!(),
        Op::NewObj => todo!(),
        Op::NewObjEx => todo!(),
        Op::Proto => {
          vm.version = buf[offset+1];  
          offset = offset + 2;
        }
        Op::Stop => break,
        Op::Frame => {
          let bytes: [u8; 8] = buf[offset+1..offset+9].try_into().unwrap();
          let frame_size = u64::from_le_bytes(bytes);
          println!("STARTING NEW FRAME WITH SIZE {}", frame_size);
          offset = offset + 9;
        },
        Op::Persid => todo!(),
        Op::BinPersid => todo!(),
        Op::ByteArray8 => todo!(),
        Op::NextBuffer => todo!(),
        Op::ReadonlyBuffer => todo!(),
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
