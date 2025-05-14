use yson::{Node, Value};
use std::collections::HashMap;
use yson::formatter::{TextFormatter, to_string, PrettyFormatter};

#[test]
fn test_node_serialize() {
    let mut attr_map = HashMap::new();
    attr_map.insert(
        "key".to_string(),
        Node {
            value: Value::String("value".to_string()),
            attributes: None,
        },
    );

    let attributes = Some(Box::new(Node {
        value: Value::Map(attr_map),
        attributes: None,
    }));

    // Основной узел: список из int и bool
    let node = Node {
        attributes,
        value: Value::List(vec![
            Node {
                value: Value::Int64(42),
                attributes: None,
            },
            Node {
                value: Value::Bool(true),
                attributes: None,
            },
            Node {
                value: Value::Null,
                attributes: None,
            },
        ]),
    };

    let mut formatter = TextFormatter;
    let s = to_string(&mut formatter, &node).unwrap();
    assert_eq!(s, r#"<"key"="value">[42;%true;#]"#);

}

#[test]
fn test_node_serialize_pretty() {
    let mut attr_map = HashMap::new();
    attr_map.insert(
        "key".to_string(),
        Node {
            value: Value::String("value".to_string()),
            attributes: None,
        },
    );

    let attributes = Some(Box::new(Node {
        value: Value::Map(attr_map),
        attributes: None,
    }));

    let node = Node {
        attributes,
        value: Value::List(vec![
            Node {
                value: Value::Int64(42),
                attributes: None,
            },
            Node {
                value: Value::Bool(true),
                attributes: None,
            },
            Node {
                value: Value::Null,
                attributes: None,
            },
        ]),
    };

    let mut formatter = PrettyFormatter::new();
    let s = to_string(&mut formatter, &node).unwrap();
    assert_eq!(
        s.trim(),
        r#"
<
  "key" = "value"
> [
  42;
  %true;
  #
]"#.trim());
}
