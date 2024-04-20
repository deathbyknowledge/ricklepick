use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    UInt(u64),
    Float(f64),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Mark,
}

impl Value {
    // fn into_bytes(&self) -> &'static[u8] {
    //     match self {
    //         Value::String(s) => {let s = s.clone(); s.as_bytes().clone()},
    //         Value::Int(v) => {
    //             let v = v.clone();
    //             &v.to_le_bytes().clone()
    //         } ,
    //         Value::Float(v) => &v.to_le_bytes(),
    //         Value::Tuple(vec) => todo!(),//vec.iter_mut().flat_map(|v| v.as_bytes()).collect(),
    //         Value::List(_) => todo!(),
    //     }
    // }

    fn size(self) -> usize {
        return match self {
            Value::String(s) => s.len(),
            Value::Int(_) => mem::size_of::<i64>(),
            Value::UInt(_) => mem::size_of::<u64>(),
            Value::Float(_) => mem::size_of::<f64>(),
            Value::Tuple(vec) => vec.len() * mem::size_of::<Value>(),
            Value::List(vec) => vec.len() * mem::size_of::<Value>(),
            Value::Mark => mem::size_of::<Value>()
        };
    }
}
enum FloatValueType {
    Raw,
    Inf,
    NInf,
    Nan,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    raw: f64,
}
