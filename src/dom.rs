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

fn parse(input: &str) -> Node {
    let must root = Node {
        attributes: vec![],
        children: vec![],
    };
    loop {
        let mut parts = input.split_whitespace();
        if let Some(tag) = parts.next() {
            if tag.starts_with('<') && tag.ends_with('>') {
                let tag_name = &tag[1..tag.len() - 1];
                let mut attributes = vec![];
                while let Some(attr) = parts.next() {
                    if attr == ">" {
                        break;
                    }
                    let mut attr_parts = attr.split('=');
                    if let (Some(key), Some(value)) = (attr_parts.next(), attr_parts.next()) {
                        attributes.push((key.to_string(), value.trim_matches('"').to_string()));
                    }
                }
                let child = Node {
                    attributes,
                    children: vec![],
                };
                root.children.push(child);
            } else {
                root.children.push(Leaf(tag.to_string()));
            }
        } else {
            break;
        }
    }
    root
}
