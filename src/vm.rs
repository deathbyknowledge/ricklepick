use std::collections::HashMap;
use std::io::Read;

use crate::op::*;

use crate::value::Value;

pub struct VM<'a> {
    // Entire Program.
    reader: &'a mut dyn Read,
    // Either current frame or entire pickle file.
    working_buffer: Box<[u8]>,
    // Program counter;
    pc: usize,
    // Protocol version;
    version: u8,
    // Value stack.
    stack: Vec<Value>,
    // VM memory.
    memo: Vec<Value>,
    // Set if parsing a framed stream.
    is_framed: bool,
}

impl<'a> VM<'a> {
    // Init the VM by reading the first OP of the buffer
    // which should set the Protocol verison.
    pub fn from(r: &'a mut dyn Read) -> Self {
        let mut vm = VM {
            reader: r,
            version: 0,
            pc: 0,
            working_buffer: Box::new([]),
            stack: Vec::new(),
            memo: Vec::new(),
            is_framed: false,
        };
        // Read the first 3 bytes. They should contain
        // 2 bytes of header + 1 initial OP, which may
        // or may not be a Frame OP. It can indicate if
        // the stream is framed though.
        let mut buf = [0; 3];
        vm.reader
            .read_exact(&mut buf)
            .expect("can't read first 3 bytes");
        if Op::from(buf[0]) != Op::Proto {
            panic!("Malformed pickle file. Does not start with 0x80.");
        }
        // Check Pickle protocol version.
        vm.version = match buf[1] {
            0..=5 => buf[1],
            _ => panic!("Unrecognized Pickle protocol version"),
        };
        // If the first OP is `Frame`, it means the stream
        // uses framing.
        if Op::Frame == buf[2].into() {
            vm.is_framed = true;
            let mut buf = [0; 8];
            vm.reader
                .read_exact(&mut buf)
                .expect("can't read frame size");
            let frame_size = u64::from_le_bytes(buf);
            vm.set_working_frame(frame_size as usize);
        }
        vm
    }

    // If stack has one final entry, pop it!
    pub fn result(&mut self) -> Result<Value, ()> {
        if let Some(value) = self.stack.pop() {
            match value {
                Value::Mark => return Err(()),
                _ => return Ok(value),
            }
        }
        Err(())
    }

    // Only call this method after an Op::Frame was read.
    fn set_working_frame(&mut self, frame_size: usize) {
        let mut buf = vec![0; frame_size];
        self.reader
            .read_exact(&mut buf)
            .unwrap_or_else(|_| panic!("couldn't read {frame_size} from reader"));
        self.working_buffer = buf.into_boxed_slice();
    }

    fn decode(&mut self) -> Result<(Op, Value), ()> {
        let op = self.next_op();
        if let Op::Stop = op {
            return Err(());
        }
        let arg = self.read_arg(op.clone());
        //println!("CURRENT OP {:?} WITH ARG: {:?}", op, arg);
        //println!("VM STACK CURRENTLY IS {:?}", self.stack);

        Ok((op, arg))
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.working_buffer[self.pc];
        self.pc += 1;
        byte
    }

    fn next_bytes<const L: usize>(&mut self) -> [u8; L] {
        let bytes: [u8; L] = self.working_buffer[self.pc..self.pc + L]
            .try_into()
            .unwrap();
        self.pc += L;
        bytes
    }

    fn next_op(&mut self) -> Op {
        Op::from(self.next_byte())
    }

    fn read_arg(&mut self, op: Op) -> Value {
        match op {
            Op::AddItems => todo!(),
            Op::Append => todo!(),
            Op::Appends => Value::None,
            Op::Binbytes => todo!(),
            Op::Binbytes8 => todo!(),
            Op::Binfloat => Value::Float(f64::from_be_bytes(self.next_bytes::<8>())),
            Op::BinGet => todo!(),
            Op::BinInt => Value::Int(i32::from_le_bytes(self.next_bytes::<4>())),
            Op::BinInt1 => Value::UInt(self.next_byte() as u32),
            Op::BinInt2 => Value::UInt(u16::from_le_bytes(self.next_bytes::<2>()) as u32),
            Op::Binstring => {
                let len = i32::from_le_bytes(self.next_bytes::<4>());
                let s = String::from_utf8(
                    self.working_buffer[self.pc..self.pc + (len as usize)].into(),
                )
                .expect("meow");
                self.pc += len as usize;
                Value::String(s)
            }
            Op::BinPersid => todo!(),
            Op::Binunicode => todo!(),
            Op::Binunicode8 => todo!(),
            Op::BinPut => todo!(),
            Op::Build => todo!(),
            Op::ByteArray8 => todo!(),
            Op::Dict => todo!(),
            Op::Dup => todo!(),
            Op::EmptyDict => Value::None,
            Op::EmptyList => Value::None,
            Op::EmptySet => todo!(),
            Op::EmptyTuple => todo!(),
            Op::Ext1 => todo!(),
            Op::Ext2 => todo!(),
            Op::Ext4 => todo!(),
            Op::Float => todo!(),
            Op::Frame => Value::ULong(u64::from_le_bytes(self.next_bytes::<8>())),
            Op::FrozenSet => todo!(),
            Op::Get => todo!(),
            Op::GlobalOpcode => todo!(),
            Op::Int => todo!(),
            Op::Inst => todo!(),
            Op::List => todo!(),
            Op::Long => todo!(),
            Op::Long1 => todo!(),
            Op::Long4 => todo!(),
            Op::LongBinGet => todo!(),
            Op::LongBinPut => Value::UInt(u32::from_le_bytes(self.next_bytes::<4>())),
            Op::Mark => Value::None,
            Op::Memoize => Value::None,
            Op::NewObj => todo!(),
            Op::NewObjEx => todo!(),
            Op::Newfalse => Value::None,
            Op::Newtrue => Value::None,
            Op::NextBuffer => todo!(),
            Op::None => todo!(),
            Op::Obj => todo!(),
            Op::Persid => todo!(),
            Op::Pop => todo!(),
            Op::PopMark => todo!(),
            Op::Proto => Value::UInt(self.next_byte() as u32),
            Op::Put => todo!(),
            Op::ReadonlyBuffer => todo!(),
            Op::Reduce => todo!(),
            Op::SetItem => todo!(),
            Op::SetItems => Value::None,
            Op::ShortBinbytes => todo!(),
            Op::ShortBinstring => todo!(),
            Op::ShortBinunicde => {
                let len = self.next_byte();
                let s = String::from_utf8(
                    self.working_buffer[self.pc..self.pc + (len as usize)].into(),
                )
                .expect("meow");
                self.pc += len as usize;
                Value::String(s)
            }
            Op::StackGlobal => todo!(),
            Op::Stop => todo!(),
            Op::String => todo!(),
            Op::Tuple => Value::None,
            Op::Tuple1 => todo!(),
            Op::Tuple2 => Value::None,
            Op::Tuple3 => Value::None,
            Op::Unicode => todo!(),
        }
    }

    pub fn step(&mut self) -> bool {
        if let Ok((op, arg)) = self.decode() {
            match (op, arg.clone()) {
                (Op::Appends, _) => {
                    let mut values = {
                        let mut values: Vec<Value> = Vec::new();
                        loop {
                            let v = self.stack.pop().unwrap();
                            if v == Value::Mark {
                                break;
                            }
                            values.insert(0, v);
                        }
                        values
                    };
                    if let Some(Value::List(vec)) = self.stack.last_mut() {
                        vec.append(&mut values);
                    } else {
                        panic!("Stack ordering was wrong")
                    }
                }
                (Op::BinInt1, Value::UInt(_)) => self.stack.push(arg),
                (Op::Binfloat, Value::Float(_)) => self.stack.push(arg),
                (Op::EmptyList, _) => self.stack.push(Value::List(Vec::new())),
                (Op::EmptyDict, _) => self.stack.push(Value::Dict(HashMap::new())),
                (Op::Frame, Value::ULong(_)) => self.stack.push(arg), // TODO: update
                (Op::Mark, _) => self.stack.push(Value::Mark),
                (Op::Memoize, _) => {
                    let val = self.stack.last().unwrap();
                    self.memo.push(val.clone());
                }
                (Op::Newfalse, _) => {
                    self.stack.push(Value::Bool(false));
                }
                (Op::Newtrue, _) => {
                    self.stack.push(Value::Bool(true));
                }
                (Op::SetItems, _) => {
                    let values = {
                        // Array of (K, V)
                        let mut kv_values: Vec<(Value, Value)> = Vec::new();
                        loop {
                            let v = self.stack.pop().unwrap();
                            if v == Value::Mark {
                                break;
                            }
                            let k = self.stack.pop().unwrap();
                            kv_values.insert(0, (k, v));
                        }
                        kv_values
                    };
                    if let Some(Value::Dict(map)) = self.stack.last_mut() {
                        for (k, v) in values.into_iter() {
                            map.insert(k, v);
                        }
                    } else {
                        panic!("Stack ordering was wrong")
                    }
                }
                (Op::ShortBinunicde, Value::String(_)) => self.stack.push(arg),
                // Create a tuple from all topmost values in stack
                // delimited by a Mark object.
                (Op::Tuple, _) => {
                    let values = {
                        let mut values: Vec<Value> = Vec::new();
                        loop {
                            let v = self.stack.pop().unwrap();
                            if v == Value::Mark {
                                break;
                            }
                            values.insert(0, v);
                        }
                        values
                    };
                    self.stack.push(Value::Tuple(values));
                }
                (Op::Tuple2, _) => {
                    let b = self.stack.pop().expect("meow");
                    let a = self.stack.pop().expect("meow");
                    self.stack.push(Value::Tuple(vec![a, b]));
                }
                (Op::Tuple3, _) => {
                    let c = self.stack.pop().expect("meow");
                    let b = self.stack.pop().expect("meow");
                    let a = self.stack.pop().expect("meow");
                    self.stack.push(Value::Tuple(vec![a, b, c]));
                }
                _ => unimplemented!(),
            }
            return true;
        }
        false
    }
}
