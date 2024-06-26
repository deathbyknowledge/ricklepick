// https://github.com/python/cpython/blob/3.12/Lib/pickletools.py

pub const MARK: u8 = 40;
pub const EMPTY_TUPLE: u8 = 41;
pub const STOP: u8 = 46;
pub const POP: u8 = 48;
pub const POP_MARK: u8 = 49;
pub const DUP: u8 = 50;
pub const BINBYTES: u8 = 66;
pub const SHORT_BINBYTES: u8 = 67;
pub const FLOAT: u8 = 70;
pub const BINFLOAT: u8 = 71;
pub const INT: u8 = 73;
pub const BININT: u8 = 74;
pub const BININT1: u8 = 75;
pub const LONG: u8 = 76;
pub const BININT2: u8 = 77;
pub const NONE: u8 = 78;
pub const PERSID: u8 = 80;
pub const BINPERSID: u8 = 81;
pub const REDUCE: u8 = 82;
pub const STRING: u8 = 83;
pub const BINSTRING: u8 = 84;
pub const SHORT_BINSTRING: u8 = 85;
pub const UNICODE: u8 = 86;
pub const BINUNICODE: u8 = 88;
pub const EMPTY_LIST: u8 = 93;
pub const APPEND: u8 = 97;
pub const BUILD: u8 = 98;
pub const GLOBAL_OPCODE: u8 = 99;
pub const DICT: u8 = 100;
pub const APPENDS: u8 = 101;
pub const GET: u8 = 103;
pub const BINGET: u8 = 104;
pub const INST: u8 = 105;
pub const LONG_BINGET: u8 = 106;
pub const LIST: u8 = 108;
pub const OBJ: u8 = 111;
pub const PUT: u8 = 112;
pub const BINPUT: u8 = 113;
pub const LONG_BINPUT: u8 = 114;
pub const SET_ITEM: u8 = 115;
pub const TUPLE: u8 = 116;
pub const SET_ITEMS: u8 = 117;
pub const EMPTY_DICT: u8 = 125;
pub const PROTO: u8 = 128;
pub const NEW_OBJ: u8 = 129;
pub const EXT1: u8 = 130;
pub const EXT2: u8 = 131;
pub const EXT4: u8 = 132;
pub const TUPLE1: u8 = 133;
pub const TUPLE2: u8 = 134;
pub const TUPLE3: u8 = 135;
pub const NEWTRUE: u8 = 136;
pub const NEWFALSE: u8 = 137;
pub const LONG1: u8 = 138;
pub const LONG4: u8 = 139;
pub const SHORT_BINUNICODE: u8 = 140;
pub const BINUNICODE8: u8 = 141;
pub const BINBYTES8: u8 = 142;
pub const EMPTY_SET: u8 = 143;
pub const ADD_ITEMS: u8 = 144;
pub const FROZEN_SET: u8 = 145;
pub const NEW_OBJ_EX: u8 = 146;
pub const STACK_GLOBAL: u8 = 147;
pub const MEMOIZE: u8 = 148;
pub const FRAME: u8 = 149;
pub const BYTEARRAY8: u8 = 150;
pub const NEXT_BUFFER: u8 = 151;
pub const READONLY_BUFFER: u8 = 152;

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Int,
    BinInt,
    BinInt1,
    BinInt2,
    Long,
    Long1,
    Long4,
    String,
    BinString,
    ShortBinstring,
    BinBytes,
    ShortBinbytes,
    BinBytes8,
    None,
    NewTrue,
    NewFalse,
    Unicode,
    ShortBinunicde,
    BinUnicode,
    BinUnicode8,
    Float,
    BinFloat,
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

impl From<Op> for u8 {
    fn from(op: Op) -> u8 {
        match op {
            Op::Int => INT,
            Op::BinInt => BININT,
            Op::BinInt1 => BININT1,
            Op::BinInt2 => BININT2,
            Op::Long => LONG,
            Op::Long1 => LONG1,
            Op::Long4 => LONG4,
            Op::String => STRING,
            Op::BinString => BINSTRING,
            Op::ShortBinstring => SHORT_BINSTRING,
            Op::BinBytes => BINBYTES,
            Op::ShortBinbytes => SHORT_BINBYTES,
            Op::BinBytes8 => BINBYTES8,
            Op::None => NONE,
            Op::NewTrue => NEWTRUE,
            Op::NewFalse => NEWFALSE,
            Op::Unicode => UNICODE,
            Op::ShortBinunicde => SHORT_BINUNICODE,
            Op::BinUnicode => BINUNICODE,
            Op::BinUnicode8 => BINUNICODE8,
            Op::Float => FLOAT,
            Op::BinFloat => BINFLOAT,
            Op::EmptyList => EMPTY_LIST,
            Op::Append => APPEND,
            Op::Appends => APPENDS,
            Op::List => LIST,
            Op::EmptyTuple => EMPTY_TUPLE,
            Op::Tuple => TUPLE,
            Op::Tuple1 => TUPLE1,
            Op::Tuple2 => TUPLE2,
            Op::Tuple3 => TUPLE3,
            Op::EmptyDict => EMPTY_DICT,
            Op::Dict => DICT,
            Op::SetItem => SET_ITEM,
            Op::SetItems => SET_ITEMS,
            Op::EmptySet => EMPTY_SET,
            Op::AddItems => ADD_ITEMS,
            Op::FrozenSet => FROZEN_SET,
            Op::Pop => POP,
            Op::Dup => DUP,
            Op::Mark => MARK,
            Op::PopMark => POP_MARK,
            Op::Get => GET,
            Op::BinGet => BINGET,
            Op::LongBinGet => LONG_BINGET,
            Op::Put => PUT,
            Op::BinPut => BINPUT,
            Op::LongBinPut => LONG_BINPUT,
            Op::Memoize => MEMOIZE,
            Op::Ext1 => EXT1,
            Op::Ext2 => EXT2,
            Op::Ext4 => EXT4,
            Op::GlobalOpcode => GLOBAL_OPCODE,
            Op::StackGlobal => STACK_GLOBAL,
            Op::Reduce => REDUCE,
            Op::Build => BUILD,
            Op::Inst => INST,
            Op::Obj => OBJ,
            Op::NewObj => NEW_OBJ,
            Op::NewObjEx => NEW_OBJ_EX,
            Op::Proto => PROTO,
            Op::Stop => STOP,
            Op::Frame => FRAME,
            Op::Persid => PERSID,
            Op::BinPersid => BINPERSID,
            Op::ByteArray8 => BYTEARRAY8,
            Op::NextBuffer => NEXT_BUFFER,
            Op::ReadonlyBuffer => READONLY_BUFFER,
        }
    }
}

impl From<u8> for Op {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            INT => Op::Int,
            BININT => Op::BinInt,
            BININT1 => Op::BinInt1,
            BININT2 => Op::BinInt2,
            LONG => Op::Long,
            LONG1 => Op::Long1,
            LONG4 => Op::Long4,
            STRING => Op::String,
            BINSTRING => Op::BinString,
            SHORT_BINSTRING => Op::ShortBinstring,
            BINBYTES => Op::BinBytes,
            SHORT_BINBYTES => Op::ShortBinbytes,
            BINBYTES8 => Op::BinBytes8,
            NONE => Op::None,
            NEWTRUE => Op::NewTrue,
            NEWFALSE => Op::NewFalse,
            UNICODE => Op::Unicode,
            SHORT_BINUNICODE => Op::ShortBinunicde,
            BINUNICODE => Op::BinUnicode,
            BINUNICODE8 => Op::BinUnicode8,
            FLOAT => Op::Float,
            BINFLOAT => Op::BinFloat,
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
            0..=u8::MAX => panic!("Can't parse unknown opcode {}", value),
        }
    }
}
