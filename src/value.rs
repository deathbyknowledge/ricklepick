use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i32),
    UInt(u32),
    Long(i128),
    ULong(u128),
    Float(f64),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Dict(HashMap<Value, Value>),
    Bytes(Vec<u8>),
    Object(Instance),
    Callable(Instance, Box<Value>),
    Mark,
    None,
}

impl Value {
    pub fn as_bool(self) -> Option<bool> {
        if let Self::Bool(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_string(self) -> Option<String> {
        if let Self::String(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_uint(self) -> Option<u32> {
        if let Self::UInt(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_int(self) -> Option<i32> {
        if let Self::Int(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_ulong(self) -> Option<u128> {
        if let Self::ULong(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_long(self) -> Option<i128> {
        if let Self::Long(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_bytes(self) -> Option<Vec<u8>> {
        if let Self::Bytes(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn as_instance(self) -> Option<Instance> {
        if let Self::Object(x) = self {
            Some(x)
        } else {
            None
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    name: String,
    module: String,
    fields: HashMap<String, Value>,
    pub args: Vec<Value>,
    pub kwargs: Option<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(name: String, module: String) -> Self {
        Instance {
            name,
            module,
            fields: HashMap::new(),
            args: Vec::new(),
            kwargs: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn module(&self) -> String {
        self.module.clone()
    }

    pub fn as_key(&self) -> String {
        format!("{}.{}", self.module, self.name)
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
        s
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
            (Value::Bytes(a), Value::Bytes(b)) => {
                if a.len() == b.len() {
                    self.to_string() == other.to_string()
                } else {
                    false
                }
            }
            (Value::Callable(f1, arg1), Value::Callable(f2, arg2)) => *f1 == *f2 && arg1 == arg2,
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
            Value::Bool(v) => write!(f, "{}", if *v { "True" } else { "False" }),
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
            Value::Bytes(v) => {
                let s = v
                    .iter()
                    .map(|i| format!("{i:x}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{s}]")
            }
            Value::Object(inst) => write!(
                f,
                "<{} object at {:p}> (fields: {:?}, args: {:?}, kwargs: {:?})",
                inst.as_key(),
                inst,
                inst.fields_to_string(),
                inst.args,
                inst.kwargs
            ),
            Value::Callable(inst, arg) => write!(f, "*{}({})", inst.as_key(), arg),
            Value::Mark => write!(f, "Mark"),
            Value::None => write!(f, "None"),
        }
    }
}
