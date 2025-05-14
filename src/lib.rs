use std::vec::Vec;
use std::collections::HashMap;

pub type MapType = HashMap<String, Node>;
pub type ListType = Vec<Node>;

/// Represents any valid YSON value.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int64(i64),
    Uint64(u64),
    Double(f64),
    Bool(bool),
    List(ListType),
    Map(MapType),
    Null,
}

/// Represents any valid YSON Node.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub attributes: Option<Box<Node>>,
    pub value: Value, 
}

pub mod formatter;
