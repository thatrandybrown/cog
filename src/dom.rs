// a generic defintion of a dom node
struct Node<T> {
    attributes: Vec<(String, String)>,
    children: Vec<Node<T>>
}

pub fn Node(attributes: Vec<(String, String)>, children: Vec<Node>) -> Node {
    Node {
        attributes,
        children,
    }
}

pub fn Leaf(value: String) -> Node {
    Node {
        attributes: vec![("value".to_string(), value)],
        children: vec![],
    }
}
