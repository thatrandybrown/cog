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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node(node: &Node, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            let indent_str = "  ".repeat(indent);
            match &node.tag {
                Some(tag) => write!(f, "{}{}:", indent_str, tag)?,
                None => write!(f, "{}Text:", indent_str)?,
            }

            if !node.attributes.is_empty() {
                write!(f, " [")?;
                for (i, (key, value)) in node.attributes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}=\"{}\"", key, value)?;
                }
                write!(f, "]")?;
            }
            writeln!(f)?;

            for child in &node.children {
                print_node(child, f, indent + 1)?;
            }
            Ok(())
        }
        print_node(self, f, 0)
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
            let child = Node::new(Some(tag_name), attributes, vec![]);
            root.children.push(child);
        } else {
            text_content.push_str(part);
            text_content.push(' ');
        }
    }

    if !text_content.is_empty() {
        root.children.push(Node::new(None, vec![("value".to_string(), text_content.trim().to_string())], vec![]));
    }

    root
}

pub fn main() {
    let input = r#"<html lang="en">
        <head>
            <title>My Web Page</title>
        </head>
        <body>
            <h1>Welcome to my web page</h1>
            <p>This is a paragraph with <a href="https://www.example.com">a link</a>.</p>
        </body>
    </html>"#;

    let parsed_tree = parse(input);
    println!("Parsed DOM Tree:");
    println!("{}", parsed_tree);
}
