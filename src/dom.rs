use std::rc::{Rc, Weak};
use std::cell::RefCell;

use std::fmt;

#[derive(Debug, Clone)]
struct Node {
    tag: Option<String>,
    attributes: Vec<(String, String)>,
    children: Vec<Node>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(tag: Option<String>, attributes: Vec<(String, String)>, children: Vec<Node>) -> Self {
        Node {
            tag,
            attributes,
            children,
            parent: None,
        }
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
    let mut root: Option<Rc<RefCell<Node>>> = None;
    let mut current: Option<Rc<RefCell<Node>>> = None;
    let mut text_content = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            if !text_content.trim().is_empty() {
                current.as_ref().unwrap().borrow_mut().children.push(Node::new(None, vec![("value".to_string(), text_content.trim().to_string())], vec![]));
                text_content.clear();
            }

            let mut tag = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_whitespace() || next_c == '>' {
                    // opening tag found
                    // set tag to root node
                    break;
                }
                tag.push(chars.next().unwrap());
            }

            if tag.starts_with('/') {
                // consume to the end of the tag
                while let Some(&next_c) = chars.peek() {
                    if next_c == '>' {
                        chars.next();
                        break;
                    }
                    chars.next();
                }
                continue;
            }

            let mut attributes = vec![];
            while let Some(&next_c) = chars.peek() {
                if next_c == '>' {
                    chars.next();
                    break;
                }
                if next_c.is_whitespace() {
                    chars.next();
                    continue;
                }
                let (key, value) = parse_attribute(&mut chars);
                attributes.push((key, value));
            }

            let mut child = Rc::new(RefCell::new(Node::new(Some(tag), attributes, vec![])));
            // if root is not initialized, initialize it
            if root.is_none() {
                root = Some(child);
                current = root.clone();
            } else {
                child.borrow_mut().parent = Some(Rc::downgrade(current.as_ref().unwrap()));
                // child.borrow_mut().parent = Some(Rc::downgrade(&root));
                current.as_ref().unwrap().borrow_mut().children.push(Rc::try_unwrap(child).expect("Failed to unwrap Rc").into_inner());
            }
        } else {
            text_content.push(c);
        }
    }

    if !text_content.trim().is_empty() {
        current.as_ref().unwrap().borrow_mut().children.push(Node::new(None, vec![("value".to_string(), text_content.trim().to_string())], vec![]));
    }

    drop(current); // Drop current to avoid holding onto the last node
    match root {
        Some(rc_node) => Rc::try_unwrap(rc_node)
            .expect("Failed to unwrap Rc")
            .into_inner(),
        None => Node::new(None, vec![], vec![])
    }
}

fn parse_attribute(chars: &mut std::iter::Peekable<std::str::Chars>) -> (String, String) {
    let mut key = String::new();
    let mut value = String::new();

    // Parse key
    while let Some(&c) = chars.peek() {
        if c == '=' {
            chars.next();
            break;
        }
        key.push(chars.next().unwrap());
    }

    // Parse value
    if let Some(&quote) = chars.peek() {
        if quote == '"' || quote == '\'' {
            chars.next(); // consume opening quote
            while let Some(&c) = chars.peek() {
                if c == quote {
                    chars.next(); // consume closing quote
                    break;
                }
                value.push(chars.next().unwrap());
            }
        } else {
            // Unquoted value
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() || c == '>' {
                    break;
                }
                value.push(chars.next().unwrap());
            }
        }
    }

    (key.trim().to_string(), value.to_string())
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
