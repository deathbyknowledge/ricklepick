use std::fmt::Display;

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
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "'{s}'"),
            Value::Int(v) => write!(f, "{v}"),
            Value::UInt(v) => write!(f, "{v}"),
            Value::Long(v) => write!(f, "{v}"),
            Value::ULong(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v:.1}"),
            Value::Tuple(v) => {
                let s = v
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "({s})")
            }
            Value::List(v) => {
                let s = v
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{s}]")
            }
            Value::Mark => write!(f, "Mark"),
            Value::None => write!(f, "None"),
        }
    }
}
