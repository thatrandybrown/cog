use std::rc::{Rc, Weak};
use std::cell::RefCell;

use std::fmt;

#[derive(Debug)]
struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selector: Vec<String>,
    declarations: Vec<(String, String)>, // Vec<Declaration>,
}

struct Declaration {
    property: String,
    value: String,
}

#[derive(Debug)]
struct Node {
    tag: Option<String>,
    attributes: Vec<(String, String)>,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
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
                print_node(&child.borrow(), f, indent + 1)?;
            }
            Ok(())
        }
        print_node(self, f, 0)
    }
}

impl fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.rules {
            for selector_part in &rule.selector {
                write!(f, "{}", selector_part)?;
                write!(f, ":\t")?;
                write!(f, " [")?;
                for (i, (key, value)) in rule.declarations.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}=\"{}\"", key, value)?;
                }
                write!(f, "]")?;
                writeln!(f)?;
            }
        }
        Ok(())
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

fn parse_html(input: &str) -> Node {
    let mut root: Option<Rc<RefCell<Node>>> = None;
    let mut current: Option<Rc<RefCell<Node>>> = None;
    let mut text_content = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            if !text_content.trim().is_empty() {
                current.as_ref().unwrap().borrow_mut().children.push(Rc::new(RefCell::new(Node { tag: None, attributes: vec![("value".to_string(), text_content.trim().to_string())], children: vec![], parent: None })));
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
                // closing tag - move up to parent
                if let Some(current_node) = &current {
                    let parent_rc = current_node.borrow().parent.as_ref()
                        .and_then(|parent_weak| parent_weak.upgrade());

                    if let Some(parent) = parent_rc {
                        current = Some(parent);
                    }
                }
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

            let node = Node { tag: Some(tag), attributes, children: vec![], parent: None };
            let child = Rc::new(RefCell::new(node));
            // if root is not initialized, initialize it
            if root.is_none() {
                root = Some(child);
                current = root.clone();
            } else {
                child.borrow_mut().parent = Some(Rc::downgrade(current.as_ref().unwrap()));
                // child.borrow_mut().parent = Some(Rc::downgrade(&root));
                current.as_ref().unwrap().borrow_mut().children.push(child.clone());
                current = Some(child);
            }
        } else {
            text_content.push(c);
        }
    }

    if !text_content.trim().is_empty() {
        current.as_ref().unwrap().borrow_mut().children.push(Rc::new(RefCell::new(Node { tag: None, attributes: vec![("value".to_string(), text_content.trim().to_string())], children: vec![], parent: None })));
    }

    drop(current); // Drop current to avoid holding onto the last node
    match root {
        Some(rc_node) => Rc::try_unwrap(rc_node)
            .expect("Failed to unwrap Rc")
            .into_inner(),
        None => Node { tag: None, attributes: vec![], children: vec![], parent: None }
    }
}

fn parse_css(input: &str) -> Stylesheet {
    let mut rules = vec![];
    let mut chars = input.chars().peekable();
    let mut selector = String::new();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut declaration = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c == '}' {
                    chars.next();
                    break;
                } else if next_c.is_whitespace() {
                    chars.next();
                    continue;
                }
                declaration.push(chars.next().unwrap());
            }
            // split declaration into list of declarations split at ';'
            let declarations: Vec<(String, String)> = declaration.split(';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut parts = s.split(':');
                    let key = parts.next().unwrap_or("").trim().to_string();
                    let value = parts.next().unwrap_or("").trim().to_string();
                    (key, value)
                })
                .collect();
            rules.push(Rule { selector: selector.clone().trim().split(",").map(|s| s.trim().to_string()).collect(), declarations });
            selector.clear();
        } else {
            selector.push(c);
        }
    }

    Stylesheet { rules }
}

pub fn main() {
    const HTML : &str = r#"
        <html>
            <head>
                <title>Test Document</title>
            </head>
            <body>
                <h1 class="header">Hello, World!</h1>
                <p>This is a <a href="">link</a> in a paragraph.</p>
            </body>
        </html>
    "#;

    let dom = parse_html(HTML);
    println!("{}", dom);

    const CSS: &str = r#"
        body {
            font-family: Arial, sans-serif;
            background-color: #f4f4f4;
        }

        h1 {
            color: #333;
        }

        p {
            line-height: 1.5;
        }
        body .header, h1{
            font-size: 24px;
            font-weight: bold;
        }
    "#;

    let cssom = parse_css(CSS);
    println!("{}", cssom);

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <html_input>", args[0]);
        return;
    }

    let parsed_tree = parse_html(args[1].as_str());

    println!("{:?}", parsed_tree);
}
