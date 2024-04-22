use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f64),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Dict(HashMap<Value, Value>),
    Object(Instance),
    Mark,
    None,
}

#[derive(Debug, Clone)]
pub struct Instance{
    name: String,
    module: String,
    fields: HashMap<String, Value>,
    pub args: Vec<Value>,
    pub kwargs: Option<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(name: String, module: String) -> Self {
        Instance {name, module, fields: HashMap::new(), args: Vec::new(), kwargs: None }
    }

    pub fn set_fields(&mut self, new_fields: HashMap<Value, Value>) {
        for (k, v) in new_fields {
            if let Value::String(k) = k {
                self.fields.insert(k, v);
            } else {
                panic!("tried using a non string for an obj field name");
            }
        }
    }

    pub fn fields_to_string(&self) -> String {
        let mut vec: Vec<_> = self.fields.iter().collect();
        vec.sort_by_key(|&(k, _)| k.clone());
        let s = vec
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>()
            .join(", ");
        return s;
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::UInt(a), Value::UInt(b)) => a == b,
            (Value::Long(a), Value::Long(b)) => a == b,
            (Value::ULong(a), Value::ULong(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => {
                if a.len() == b.len() {
                    self.to_string() == other.to_string()
                } else {
                    false
                }
            }
            (Value::List(a), Value::List(b)) => {
                if a.len() == b.len() {
                    self.to_string() == other.to_string()
                } else {
                    false
                }
            }
            (Value::Dict(a), Value::Dict(b)) => {
                if a.len() == b.len() {
                    self.to_string() == other.to_string()
                } else {
                    false
                }
            }
            (Value::Mark, Value::Mark) => true,
            (Value::None, Value::None) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => write!(f,"{}", if *v {"True"} else {"False"}),
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
            Value::Dict(v) => {
                let mut vec: Vec<_> = v.iter().collect();
                vec.sort_by_key(|&(k, _)| k.to_string().clone());
                let s = vec
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{s}}}")
            }
            Value::List(v) => {
                let s = v
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{s}]")
            }
            Value::Object(inst) => write!(f, "<{}.{} object at {:p}> (fields: {:?}, args: {:?}, kwargs: {:?})", inst.module, inst.name, inst, inst.fields_to_string(), inst.args, inst.kwargs),
            Value::Mark => write!(f, "Mark"),
            Value::None => write!(f, "None"),
        }
    }
}
