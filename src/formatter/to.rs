use crate::{Node, Value};
use crate::formatter::Formatter;
use std::io::{self, Write};

pub fn to_writer<W: ?Sized + Write, F: Formatter>(
    writer: &mut W,
    formatter: &mut F,
    node: &Node,
) -> io::Result<()> {
    write_node(writer, formatter, node)
}

pub fn to_string<F: Formatter>(formatter: &mut F, node: &Node) -> io::Result<String> {
    let mut buf = Vec::new();
    to_writer(&mut buf, formatter, node)?;
    Ok(String::from_utf8(buf).expect("YSON must be valid UTF-8"))
}

pub fn to_vec<F: Formatter>(formatter: &mut F, node: &Node) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    to_writer(&mut buf, formatter, node)?;
    Ok(buf)
}

pub fn write_node<W: ?Sized + Write, F: Formatter>(
    writer: &mut W,
    formatter: &mut F,
    node: &Node,
) -> io::Result<()> {
    if let Some(boxed) = &node.attributes {
        if let Value::Map(map) = &boxed.value {
            formatter.begin_attributes(writer)?;
            let mut first = true;
            for (k, v) in map {
                formatter.begin_map_key(writer, first)?;
                formatter.begin_string(writer)?;
                formatter.write_string_fragment(writer, k)?;
                formatter.end_string(writer)?;
                formatter.end_map_key(writer)?;

                formatter.begin_map_value(writer)?;
                write_node(writer, formatter, v)?;
                formatter.end_map_value(writer)?;
                first = false;
            }
            formatter.end_attributes(writer)?;
        }
    }
    write_value(writer, formatter, &node.value)
}

fn write_value<W: ?Sized + Write, F: Formatter>(
    writer: &mut W,
    formatter: &mut F,
    value: &Value,
) -> io::Result<()> {
    match value {
        Value::Null => formatter.write_null(writer),
        Value::Bool(b) => formatter.write_bool(writer, *b),
        Value::Int64(i) => formatter.write_i64(writer, *i),
        Value::Uint64(u) => formatter.write_u64(writer, *u),
        Value::Double(f) => formatter.write_f64(writer, *f),
        Value::String(s) => {
            formatter.begin_string(writer)?;
            formatter.write_string_fragment(writer, s)?;
            formatter.end_string(writer)
        }
        Value::List(elements) => {
            formatter.begin_list(writer)?;
            let mut first = true;
            for elem in elements {
                formatter.begin_list_value(writer, first)?;
                write_node(writer, formatter, elem)?;
                formatter.end_list_value(writer)?;
                first = false;
            }
            formatter.end_list(writer)
        }
        Value::Map(map) => {
            formatter.begin_map(writer)?;
            let mut first = true;
            for (k, v) in map {
                formatter.begin_map_key(writer, first)?;
                formatter.begin_string(writer)?;
                formatter.write_string_fragment(writer, k)?;
                formatter.end_string(writer)?;
                formatter.end_map_key(writer)?;

                formatter.begin_map_value(writer)?;
                write_node(writer, formatter, v)?;
                formatter.end_map_value(writer)?;
                first = false;
            }
            formatter.end_map(writer)
        }
    }
}