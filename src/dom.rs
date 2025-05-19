// a generic defintion of a dom node
struct DomNode<T> {
    tag: String,
    attributes: Vec<(String, String)>,
    children: Vec<DomNode<T>>,
    text: Option<String>,
}
