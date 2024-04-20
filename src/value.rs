#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f64),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Mark,
    None
}
