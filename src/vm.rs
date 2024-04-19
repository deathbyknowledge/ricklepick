use std::io::BufRead;

use crate::op::*;

use crate::value::Value;

struct Instruction {
    op: Op,
    arg: OpArg,
}
impl Instruction {
    fn new(op: Op, arg: OpArg) -> Self {
        Self { op, arg }
    }
}

pub struct VM<'a> {
    // Entire Program.
    reader: &'a mut dyn BufRead,
    // Either current frame or entire pickle file.
    working_buffer: Box<[u8]>,
    // Program counter;
    pc: usize,
    // Protocol version;
    version: u8,
    // Value stack.
    stack: Vec<Value>,
    // VM memory.
    mem: [u8; 0xFFFF],
    // Set if parsing a framed stream.
    is_framed: bool,

    ops: Vec<Instruction>,
}
impl<'a> VM<'a> {
    // Init the VM by reading the first OP of the buffer
    // which should set the Protocol verison.
    pub fn from(r: &'a mut dyn BufRead) -> Self {
        let mut vm = VM {
            reader: r,
            version: 0,
            pc: 0,
            working_buffer: Box::new([]),
            stack: Vec::new(),
            mem: [0; 0xFFFF],
            is_framed: false,
            ops: Vec::new(),
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
        vm.ops
            .push(Instruction::new(Op::Proto, OpArg::U1(vm.version)));
        // If the first OP is `Frame`, it means the stream
        // uses framing.
        if Op::from_u8(buf[2]) == Op::Frame {
            vm.is_framed = true;
            let mut buf = [0; 8];
            vm.reader
                .read_exact(&mut buf)
                .expect("can't read frame size");
            let frame_size = u64::from_le_bytes(buf);
            vm.ops
                .push(Instruction::new(Op::Frame, OpArg::U8(frame_size)));
            vm.set_working_frame(frame_size as usize);
        }
        vm
    }

    // Only call this method after an Op::Frame was read.
    fn set_working_frame(&mut self, frame_size: usize) {
        let mut buf = vec![0; frame_size];
        self.reader
            .read_exact(&mut buf)
            .expect(format!("couldn't read {frame_size} from reader").as_str());
        self.working_buffer = buf.into_boxed_slice();
    }

    fn decode(&mut self) -> Result<(Op, OpArg), ()> {
        let op = self.next_op();
        if let Op::Stop = op {
            return Err(());
        }
        let arg = self.read_arg(op.clone());
        println!("CURRENT OP {:?} WITH ARG: {:?}", op, arg);

        self.ops.push(Instruction::new(op.clone(), arg.clone()));
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

    fn read_arg(&mut self, op: Op) -> OpArg {
        match op {
            Op::Int => OpArg::DecimalNlShort(2),
            Op::BinInt => todo!(),
            Op::BinInt1 => OpArg::U1(self.next_byte()),
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
            Op::Tuple3 => OpArg::NoArg,
            Op::EmptyDict => todo!(),
            Op::Dict => todo!(),
            Op::SetItem => todo!(),
            Op::SetItems => todo!(),
            Op::EmptySet => todo!(),
            Op::AddItems => todo!(),
            Op::FrozenSet => todo!(),
            Op::Pop => todo!(),
            Op::Dup => todo!(),
            Op::Mark => OpArg::NoArg,
            Op::PopMark => todo!(),
            Op::Get => todo!(),
            Op::BinGet => todo!(),
            Op::LongBinGet => todo!(),
            Op::Put => todo!(),
            Op::BinPut => todo!(),
            Op::LongBinPut => OpArg::U4(u32::from_le_bytes(self.next_bytes::<4>())),
            Op::Memoize => OpArg::NoArg,
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
            Op::Proto => OpArg::U1(self.next_byte()),
            Op::Stop => todo!(),
            Op::Frame => OpArg::U8(u64::from_le_bytes(self.next_bytes::<8>())),
            Op::Persid => todo!(),
            Op::BinPersid => todo!(),
            Op::ByteArray8 => todo!(),
            Op::NextBuffer => todo!(),
            Op::ReadonlyBuffer => todo!(),
        }
    }

    pub fn step(&mut self) -> bool {
        if let Ok((op, arg)) = self.decode() {
            match (op, arg) {
                (Op::Tuple3, OpArg::NoArg) => {
                    let c = self.stack.pop().expect("meow");
                    let b = self.stack.pop().expect("meow");
                    let a = self.stack.pop().expect("meow");
                    self.stack.push(Value::Tuple(vec![a, b, c]));
                }
                (Op::Frame, OpArg::U8(v)) => self.stack.push(Value::Int(v)),
                (Op::Mark, OpArg::NoArg) => println!("MARKING"),
                (Op::LongBinPut, OpArg::U4(v)) => self.mem[v as usize] = self.stack.len() as u8,
                (Op::Memoize, OpArg::NoArg) => {
                    let val = self.stack.pop().unwrap();
                    self.mem[0];
                }
                (Op::BinInt1, OpArg::U1(v)) => self.stack.push(Value::Int(v as u64)),
                _ => unimplemented!(),
            }
            return true;
        }
        return false;
    }
}
