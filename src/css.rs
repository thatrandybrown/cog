struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selector: String,
    declarations: Vec<Declaration>,
}

struct Declaration {
    property: String,
    value: String,
}
