use std::collections::HashMap;
use std::io::Read;

use crate::op::*;

use crate::value::{Instance, Value};

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
    // Extensions. Used to define replacemnt for python functions.
    extensions: HashMap<String, Extension>
}

pub type Extension = fn(Value) -> Value;

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
            extensions: HashMap::new(),
        };
        // Read the first 3 bytes. They should contain
        // 2 bytes of header + 1 initial OP, which may
        // or may not be a Frame OP. It can indicate if
        // the stream is framed though.
        let mut buf = [0; 2];
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
        vm
    }

    #[inline]
    pub fn load_extension(&mut self, module: &str, name: &str, ext: Extension) {
        self.extensions.insert(format!("{}.{}", module, name), ext);
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
        self.pc = 0;
    }

    fn decode(&mut self) -> Result<(Op, Value), ()> {
        let op = self.next_op();
        if let Op::Stop = op {
            return Err(());
        }
        let arg = self.read_arg(op.clone());

        Ok((op, arg))
    }

    fn next_byte(&mut self) -> u8 {
        let byte = {
            if self.is_framed {
                self.working_buffer[self.pc]
            } else {
                let mut buf = [0; 1];
                self.reader.read_exact(&mut buf).expect("prra");
                buf[0]
            }
        };
        self.pc += 1;
        byte
    }

    fn next_bytes<const L: usize>(&mut self) -> [u8; L] {
        let bytes: [u8; L] = {
            if self.is_framed {
                self.working_buffer[self.pc..self.pc + L]
                .try_into()
                .unwrap()
            } else {
                let mut buf = [0; L];
                self.reader.read_exact(&mut buf).expect("prra");
                buf
            }
        };
        self.pc += L;
        bytes
    }

    fn next_op(&mut self) -> Op {
        Op::from(self.next_byte())
    }

    pub fn read_n(&mut self, n: usize) -> Vec<u8> {
        if self.is_framed {
            let buf = self.working_buffer[self.pc..self.pc + n].into();
            self.pc += n;
            buf
        } else {
            let mut buf = vec![0; n];
            self.reader.read_exact(&mut buf.as_mut_slice()).expect("coudlnt read n");
            buf
        }
    }

    fn read_arg(&mut self, op: Op) -> Value {
        match op {
            Op::AddItems => todo!(),
            Op::Append => Value::None,
            Op::Appends => Value::None,
            Op::BinBytes => {
                let len = u32::from_le_bytes(self.next_bytes::<4>()) as usize;
                let bytes = self.read_n(len);
                Value::Bytes(bytes)
            }
            Op::BinBytes8 => todo!(),
            Op::BinFloat => Value::Float(f64::from_be_bytes(self.next_bytes::<8>())),
            Op::BinGet => Value::UInt(self.next_byte() as u32),
            Op::BinInt => Value::Int(i32::from_le_bytes(self.next_bytes::<4>())),
            Op::BinInt1 => Value::UInt(self.next_byte() as u32),
            Op::BinInt2 => Value::UInt(u16::from_le_bytes(self.next_bytes::<2>()) as u32),
            Op::BinString => {
                let len = i32::from_le_bytes(self.next_bytes::<4>());
                let s = String::from_utf8(
                    self.read_n(len as usize)
                )
                .expect("meow");
                Value::String(s)
            }
            Op::BinPersid => Value::None,
            Op::BinUnicode => {
                let len = u32::from_le_bytes(self.next_bytes::<4>());
                let s = String::from_utf8(
                    self.read_n(len as usize)
                ).expect("wrong string encoding");
                Value::String(s)
            },
            Op::BinUnicode8 => todo!(),
            Op::BinPut => Value::UInt(self.next_byte() as u32),
            Op::Build => Value::None,
            Op::ByteArray8 => todo!(),
            Op::Dict => todo!(),
            Op::Dup => todo!(),
            Op::EmptyDict => Value::None,
            Op::EmptyList => Value::None,
            Op::EmptySet => todo!(),
            Op::EmptyTuple => Value::None,
            Op::Ext1 => todo!(),
            Op::Ext2 => todo!(),
            Op::Ext4 => todo!(),
            Op::Float => todo!(),
            Op::Frame => Value::ULong(u64::from_le_bytes(self.next_bytes::<8>()) as u128),
            Op::FrozenSet => todo!(),
            Op::Get => todo!(),
            Op::GlobalOpcode => {
                let mut bytes = vec![];
                loop {
                    let byte = self.next_byte();
                    bytes.push(byte);
                    if byte == 0xA {
                        break;
                    }
                }
                loop {
                    let byte = self.next_byte();
                    if byte == 0xA {
                        break;
                    }
                    bytes.push(byte);
                }
                let s = String::from_utf8(bytes).unwrap();
                Value::String(s)
            },
            Op::Int => todo!(),
            Op::Inst => todo!(),
            Op::List => todo!(),
            Op::Long => todo!(),
            Op::Long1 => Value::UInt(self.next_byte() as u32),
            Op::Long4 => todo!(),
            Op::LongBinGet => todo!(),
            Op::LongBinPut => Value::UInt(u32::from_le_bytes(self.next_bytes::<4>())),
            Op::Mark => Value::None,
            Op::Memoize => Value::None,
            Op::NewObj => Value::None,
            Op::NewObjEx => todo!(),
            Op::NewFalse => Value::None,
            Op::NewTrue => Value::None,
            Op::NextBuffer => todo!(),
            Op::None => Value::None,
            Op::Obj => todo!(),
            Op::Persid => todo!(),
            Op::Pop => todo!(),
            Op::PopMark => todo!(),
            Op::Proto => Value::UInt(self.next_byte() as u32),
            Op::Put => todo!(),
            Op::ReadonlyBuffer => todo!(),
            Op::Reduce => Value::None,
            Op::SetItem => todo!(),
            Op::SetItems => Value::None,
            Op::ShortBinbytes => todo!(),
            Op::ShortBinstring => todo!(),
            Op::ShortBinunicde => {
                let len = self.next_byte();
                let s = String::from_utf8(
                    self.read_n(len as usize)
                )
                .expect("meow");
                Value::String(s)
            }
            Op::StackGlobal => Value::None,
            Op::Stop => todo!(),
            Op::String => todo!(),
            Op::Tuple => Value::None,
            Op::Tuple1 => Value::None,
            Op::Tuple2 => Value::None,
            Op::Tuple3 => Value::None,
            Op::Unicode => todo!(),
        }
    }

    pub fn step(&mut self) -> bool {
        if let Ok((op, arg)) = self.decode() {
            match (op, arg.clone()) {
                (Op::Append, _) => {
                    let value = self.stack.pop().expect("Must not be empty");
                    if let Some(Value::List(vec)) = self.stack.last_mut() {
                        vec.push(value);
                    } else {
                        panic!("Stack ordering was wrong");
                    }
                }
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
                        panic!("Stack ordering was wrong");
                    }
                }
                (Op::BinBytes, Value::Bytes(_)) => {
                    self.stack.push(arg);
                }
                (Op::BinInt1, Value::UInt(_)) => self.stack.push(arg),
                (Op::BinInt2, Value::UInt(_)) => self.stack.push(arg),
                (Op::BinFloat, Value::Float(_)) => self.stack.push(arg),
                (Op::BinGet, Value::UInt(idx)) => {
                    let val = self.memo.get_mut(idx as usize).unwrap().clone();
                    println!("BINGET loaded: {val}");
                    self.stack
                        .push(val);
                }
                (Op::BinPersid, _) => {
                    // Can we ignore this?
                    //println!("PERSID: {}", self.stack.last().unwrap());
                }
                (Op::BinPut, Value::UInt(idx)) => {
                    self.memo.insert(idx as usize, self.stack.last().unwrap().clone())
                }
                (Op::BinUnicode, Value::String(_)) => self.stack.push(arg),
                (Op::Build, _) => {
                    let data = self.stack.pop().expect("moo");
                    let instance = self.stack.pop().expect("moo");
                    if let (Value::Object(mut inst), Value::Dict(dict)) = (instance, data) {
                        inst.set_fields(dict);
                        self.stack.push(Value::Object(inst));
                    } else {
                        panic!("Stack ordering was wrong")
                    }
                }
                (Op::EmptyDict, _) => self.stack.push(Value::Dict(HashMap::new())),
                (Op::EmptyList, _) => self.stack.push(Value::List(Vec::new())),
                (Op::EmptyTuple, _) => self.stack.push(Value::Tuple(Vec::new())),
                (Op::Frame, Value::ULong(frame_size)) => {
                    self.is_framed = true;
                    self.set_working_frame(frame_size as usize);
                },
                (Op::GlobalOpcode, Value::String(s)) => {
                   let v: Vec<&str> = s.split('\n').collect(); 
                   self.stack.push(Value::Object(Instance::new(v[1].to_string(), v[0].to_string())));
                }
                (Op::Long1, Value::UInt(len)) => {
                    let mut bytes = self.read_n(len as usize);
                    while bytes.len() < 16 {
                        bytes.push(0);
                    }
                    let buf: [u8; 16] = bytes.try_into().expect("wow");
                    let long = i128::from_le_bytes(buf);
                    self.stack.push(Value::Long(long));
                }
                (Op::Mark, _) => self.stack.push(Value::Mark),
                (Op::Memoize, _) => {
                    let val = self.stack.last().unwrap();
                    self.memo.push(val.clone());
                }
                (Op::NewFalse, _) => {
                    self.stack.push(Value::Bool(false));
                }
                (Op::NewObj, _) => {
                    let args = self.stack.pop().expect("moo");
                    let instance = self.stack.pop().expect("moo");
                    if let (Value::Object(mut inst), Value::Tuple(args)) = (instance, args) {
                        inst.args = args;
                        self.stack.push(Value::Object(inst));
                    } else {
                        panic!("Stack ordering was wrong");
                    }
                }
                (Op::NewTrue, _) => {
                    self.stack.push(Value::Bool(true));
                }
                (Op::None, _) => self.stack.push(Value::None),
                (Op::Reduce, _) => {
                    let pytuple = self.stack.pop().expect("bark");
                    let callable = self.stack.pop().expect("bark");

                    if let (Value::Object(inst), _) = (callable, pytuple.clone()) {
                        if let Some(fnc) = self.extensions.get_mut(&inst.as_key())  {
                            println!("FOUND extension for {}", inst.as_key());
                            self.stack.push(fnc(pytuple));
                        } else {
                            println!("did not find extension for {}", inst.as_key());
                            self.stack.push(Value::Callable(inst, Box::new(pytuple)));
                        }
                    } else {
                        panic!("This should not happen.");
                    }
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
                        panic!("Stack ordering was wrong");
                    }
                }
                (Op::ShortBinunicde, Value::String(_)) => self.stack.push(arg),
                // Push a global object on the stack.
                (Op::StackGlobal, _) => {
                    let name = self.stack.pop().unwrap();
                    let module = self.stack.pop().unwrap();
                    if let (Value::String(name), Value::String(module)) = (name, module) {
                        self.stack.push(Value::Object(Instance::new(name, module)))
                    } else {
                        panic!("Stack ordering was wrong");
                    }
                }
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
                (Op::Tuple1, _) => {
                    let a = self.stack.pop().expect("meow");
                    self.stack.push(Value::Tuple(vec![a]));
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
