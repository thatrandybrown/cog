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
    let mut root = Node {
        attributes: vec![],
        children: vec![],
    };

    let mut text_content = String::new();
    let mut parts = input.split_whitespace().peekable();

    while let Some(part) = parts.next() {
        if part.starts_with('<') && part.ends_with('>') {
            // If we have accumulated text content, add it as a leaf before processing the tag
            if !text_content.is_empty() {
                root.children.push(Leaf(text_content.trim().to_string()));
                text_content.clear();
            }

            let tag_name = &part[1..part.len() - 1];
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
            // Accumulate text content
            text_content.push_str(part);
            text_content.push(' ');
        }
    }

    // Add any remaining text content as a leaf
    if !text_content.is_empty() {
        root.children.push(Leaf(text_content.trim().to_string()));
    }

    root
}
