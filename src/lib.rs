//! Lightweight string template engine with variables, conditionals, and loops.
//!
//! Zero external dependencies. Uses a simple `HashMap<String, Value>` approach.
//!
//! # Examples
//!
//! ```
//! use philiprehberger_template::{Template, Value};
//! use std::collections::HashMap;
//!
//! let tpl = Template::parse("Hello, {name}!").unwrap();
//! let mut data = HashMap::new();
//! data.insert("name".into(), Value::from("world"));
//! assert_eq!(tpl.render(&data).unwrap(), "Hello, world!");
//! ```

use std::collections::HashMap;
use std::fmt;

/// A dynamic value that can be inserted into a template.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(f64),
    /// A boolean value.
    Bool(bool),
    /// A list of values.
    List(Vec<Value>),
    /// A nested map of key-value pairs.
    Map(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Number(n) => {
                if *n == (*n as i64) as f64 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{n}")
                }
            }
            Value::Bool(b) => write!(f, "{b}"),
            Value::List(items) => {
                let parts: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", parts.join(", "))
            }
            Value::Map(_) => write!(f, "[object]"),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n as f64)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::List(v)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::String(String::new())
    }
}

impl Value {
    /// Returns `true` if this value is truthy for conditional evaluation.
    ///
    /// - `String`: truthy if non-empty
    /// - `Number`: truthy if non-zero
    /// - `Bool`: truthy if `true`
    /// - `List`: truthy if non-empty
    /// - `Map`: always truthy
    fn is_truthy(&self) -> bool {
        match self {
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => *n != 0.0,
            Value::Bool(b) => *b,
            Value::List(v) => !v.is_empty(),
            Value::Map(_) => true,
        }
    }

    /// Returns the inner `&str` if this is a `Value::String`, or `None` otherwise.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the inner `f64` if this is a `Value::Number`, or `None` otherwise.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the inner `bool` if this is a `Value::Bool`, or `None` otherwise.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the inner slice if this is a `Value::List`, or `None` otherwise.
    #[must_use]
    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner map if this is a `Value::Map`, or `None` otherwise.
    #[must_use]
    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Returns `true` if this value is a `String`.
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns `true` if this value is a `Number`.
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns `true` if this value is a `Bool`.
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns `true` if this value is a `List`.
    #[must_use]
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Returns `true` if this value is a `Map`.
    #[must_use]
    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }
}

/// An error that occurs during template parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// A block tag (`#if`, `#each`) was opened but not properly closed.
    UnmatchedBlock(String),
    /// The template contains invalid syntax.
    InvalidSyntax(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnmatchedBlock(msg) => write!(f, "unmatched block: {msg}"),
            ParseError::InvalidSyntax(msg) => write!(f, "invalid syntax: {msg}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// An error that occurs during template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    /// A required variable was not found in the data map.
    MissingVariable(String),
    /// A value had the wrong type for the operation.
    TypeError(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::MissingVariable(msg) => write!(f, "missing variable: {msg}"),
            RenderError::TypeError(msg) => write!(f, "type error: {msg}"),
        }
    }
}

impl std::error::Error for RenderError {}

/// A parsed node in the template AST.
#[derive(Debug, Clone, PartialEq)]
enum Node {
    /// Literal text.
    Text(String),
    /// A variable reference with optional filter. `{name}` or `{name|upper}`.
    Variable { path: String, filter: Option<String> },
    /// A conditional block. `{#if cond}...{/if}` with optional `{:else}`.
    Conditional { variable: String, negated: bool, body: Vec<Node>, else_body: Vec<Node> },
    /// A loop block. `{#each list}...{/each}`.
    Loop { variable: String, body: Vec<Node> },
}

/// A parsed template that can be rendered with data.
///
/// # Examples
///
/// ```
/// use philiprehberger_template::{Template, Value};
/// use std::collections::HashMap;
///
/// let tpl = Template::parse("{greeting}, {name}!").unwrap();
/// let mut data = HashMap::new();
/// data.insert("greeting".into(), Value::from("Hi"));
/// data.insert("name".into(), Value::from("Alice"));
/// assert_eq!(tpl.render(&data).unwrap(), "Hi, Alice!");
/// ```
#[derive(Debug, Clone)]
pub struct Template {
    nodes: Vec<Node>,
}

impl Template {
    /// Parse a template string into a `Template`.
    ///
    /// Returns a `ParseError` if the template contains unmatched blocks or invalid syntax.
    #[must_use = "parsing a template has no effect unless the result is used"]
    pub fn parse(template: &str) -> Result<Template, ParseError> {
        let tokens = tokenize(template)?;
        let (nodes, rest) = parse_tokens(&tokens, None)?;
        if !rest.is_empty() {
            return Err(ParseError::InvalidSyntax(
                "unexpected tokens after end of template".into(),
            ));
        }
        Ok(Template { nodes })
    }

    /// Render this template using the provided data map.
    ///
    /// Returns a `RenderError` if a required variable is missing or a type mismatch occurs.
    #[must_use = "rendering a template has no effect unless the result is used"]
    pub fn render(&self, data: &HashMap<String, Value>) -> Result<String, RenderError> {
        let mut output = String::new();
        render_nodes(&self.nodes, data, None, &mut output)?;
        Ok(output)
    }
}

/// A token from the lexer.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Text(String),
    OpenIf(String, bool),    // variable, negated
    Else,
    CloseIf,
    OpenEach(String),        // variable
    CloseEach,
    Variable(String, Option<String>),  // path, filter
}

fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut text_buf = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '}' {
            chars.next();
            // Check for escaped brace `}}`
            if chars.peek() == Some(&'}') {
                chars.next();
                text_buf.push('}');
                continue;
            }
            // Single `}` outside a tag — just literal text
            text_buf.push('}');
            continue;
        }
        if ch == '{' {
            chars.next();
            // Check for escaped brace `{{`
            if chars.peek() == Some(&'{') {
                chars.next();
                text_buf.push('{');
                continue;
            }

            // Flush text buffer
            if !text_buf.is_empty() {
                tokens.push(Token::Text(text_buf.clone()));
                text_buf.clear();
            }

            // Read until closing `}`
            let mut tag_content = String::new();
            let mut found_close = false;
            for c in chars.by_ref() {
                if c == '}' {
                    found_close = true;
                    break;
                }
                tag_content.push(c);
            }

            if !found_close {
                return Err(ParseError::InvalidSyntax(
                    "unclosed tag, expected '}'".into(),
                ));
            }

            let tag = tag_content.trim();

            if let Some(rest) = tag.strip_prefix("#if ") {
                let rest = rest.trim();
                if let Some(var) = rest.strip_prefix('!') {
                    tokens.push(Token::OpenIf(var.trim().to_string(), true));
                } else {
                    tokens.push(Token::OpenIf(rest.to_string(), false));
                }
            } else if tag == "/if" {
                tokens.push(Token::CloseIf);
            } else if tag == ":else" {
                tokens.push(Token::Else);
            } else if let Some(rest) = tag.strip_prefix("#each ") {
                tokens.push(Token::OpenEach(rest.trim().to_string()));
            } else if tag == "/each" {
                tokens.push(Token::CloseEach);
            } else if tag.is_empty() {
                return Err(ParseError::InvalidSyntax("empty tag".into()));
            } else {
                // Variable, possibly with filter
                if let Some((path, filter)) = tag.split_once('|') {
                    tokens.push(Token::Variable(
                        path.trim().to_string(),
                        Some(filter.trim().to_string()),
                    ));
                } else {
                    tokens.push(Token::Variable(tag.to_string(), None));
                }
            }
        } else {
            text_buf.push(ch);
            chars.next();
        }
    }

    if !text_buf.is_empty() {
        tokens.push(Token::Text(text_buf));
    }

    Ok(tokens)
}

fn parse_tokens<'a>(
    tokens: &'a [Token],
    closing: Option<&str>,
) -> Result<(Vec<Node>, &'a [Token]), ParseError> {
    let mut nodes = Vec::new();
    let mut remaining = tokens;

    while !remaining.is_empty() {
        match &remaining[0] {
            Token::Text(t) => {
                nodes.push(Node::Text(t.clone()));
                remaining = &remaining[1..];
            }
            Token::Variable(path, filter) => {
                nodes.push(Node::Variable {
                    path: path.clone(),
                    filter: filter.clone(),
                });
                remaining = &remaining[1..];
            }
            Token::OpenIf(var, negated) => {
                let var = var.clone();
                let negated = *negated;
                remaining = &remaining[1..];
                let (body, rest) = parse_tokens(remaining, Some("if"))?;
                // Check if we stopped at {:else}
                let (else_body, final_rest) = if !rest.is_empty() && matches!(&rest[0], Token::Else) {
                    let after_else = &rest[1..];
                    let (eb, r) = parse_tokens(after_else, Some("if_else_end"))?;
                    (eb, r)
                } else {
                    (Vec::new(), rest)
                };
                nodes.push(Node::Conditional {
                    variable: var,
                    negated,
                    body,
                    else_body,
                });
                remaining = final_rest;
            }
            Token::Else => {
                if closing == Some("if") {
                    // Return — the caller (OpenIf handler) will parse else body
                    return Ok((nodes, remaining));
                }
                return Err(ParseError::UnmatchedBlock(
                    "unexpected {:else} without matching {#if}".into(),
                ));
            }
            Token::CloseIf => {
                if closing == Some("if") || closing == Some("if_else_end") {
                    return Ok((nodes, &remaining[1..]));
                }
                return Err(ParseError::UnmatchedBlock(
                    "unexpected {/if} without matching {#if}".into(),
                ));
            }
            Token::OpenEach(var) => {
                let var = var.clone();
                remaining = &remaining[1..];
                let (body, rest) = parse_tokens(remaining, Some("each"))?;
                nodes.push(Node::Loop {
                    variable: var,
                    body,
                });
                remaining = rest;
            }
            Token::CloseEach => {
                if closing == Some("each") {
                    return Ok((nodes, &remaining[1..]));
                }
                return Err(ParseError::UnmatchedBlock(
                    "unexpected {/each} without matching {#each}".into(),
                ));
            }
        }
    }

    if let Some(block) = closing {
        return Err(ParseError::UnmatchedBlock(format!(
            "expected closing tag for {{#{block}}}"
        )));
    }

    Ok((nodes, remaining))
}

fn resolve_path<'a>(
    path: &str,
    data: &'a HashMap<String, Value>,
    current_item: Option<&'a Value>,
) -> Option<&'a Value> {
    if path == "." {
        return current_item;
    }

    let parts: Vec<&str> = path.split('.').collect();

    // First, try resolving from current_item if it's a Map
    if let Some(Value::Map(map)) = current_item {
        if let Some(val) = resolve_from_map(&parts, map) {
            return Some(val);
        }
    }

    // Then resolve from the root data
    resolve_from_map(&parts, data)
}

fn resolve_from_map<'a>(parts: &[&str], map: &'a HashMap<String, Value>) -> Option<&'a Value> {
    if parts.is_empty() {
        return None;
    }

    let val = map.get(parts[0])?;

    if parts.len() == 1 {
        return Some(val);
    }

    match val {
        Value::Map(inner) => resolve_from_map(&parts[1..], inner),
        _ => None,
    }
}

fn apply_filter(value: &str, filter: &str) -> Result<String, RenderError> {
    match filter {
        "upper" => Ok(value.to_uppercase()),
        "lower" => Ok(value.to_lowercase()),
        "trim" => Ok(value.trim().to_string()),
        _ => Err(RenderError::TypeError(format!("unknown filter: {filter}"))),
    }
}

fn render_nodes(
    nodes: &[Node],
    data: &HashMap<String, Value>,
    current_item: Option<&Value>,
    output: &mut String,
) -> Result<(), RenderError> {
    for node in nodes {
        match node {
            Node::Text(t) => output.push_str(t),
            Node::Variable { path, filter } => {
                let val = resolve_path(path, data, current_item).ok_or_else(|| {
                    RenderError::MissingVariable(path.clone())
                })?;
                let rendered = val.to_string();
                let rendered = if let Some(f) = filter {
                    apply_filter(&rendered, f)?
                } else {
                    rendered
                };
                output.push_str(&rendered);
            }
            Node::Conditional { variable, negated, body, else_body } => {
                let val = resolve_path(variable, data, current_item);
                let truthy = val.is_some_and(|v| v.is_truthy());
                let should_render = if *negated { !truthy } else { truthy };
                if should_render {
                    render_nodes(body, data, current_item, output)?;
                } else if !else_body.is_empty() {
                    render_nodes(else_body, data, current_item, output)?;
                }
            }
            Node::Loop { variable, body } => {
                let val = resolve_path(variable, data, current_item).ok_or_else(|| {
                    RenderError::MissingVariable(variable.clone())
                })?;
                match val {
                    Value::List(items) => {
                        for item in items {
                            render_nodes(body, data, Some(item), output)?;
                        }
                    }
                    _ => {
                        return Err(RenderError::TypeError(format!(
                            "expected list for #each, got {}",
                            match val {
                                Value::String(_) => "string",
                                Value::Number(_) => "number",
                                Value::Bool(_) => "bool",
                                Value::Map(_) => "map",
                                Value::List(_) => unreachable!(),
                            }
                        )));
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data_from(pairs: Vec<(&str, Value)>) -> HashMap<String, Value> {
        pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    }

    #[test]
    fn simple_variable_replacement() {
        let tpl = Template::parse("Hello, {name}!").unwrap();
        let data = data_from(vec![("name", Value::from("world"))]);
        assert_eq!(tpl.render(&data).unwrap(), "Hello, world!");
    }

    #[test]
    fn multiple_variables() {
        let tpl = Template::parse("{greeting}, {name}!").unwrap();
        let data = data_from(vec![
            ("greeting", Value::from("Hi")),
            ("name", Value::from("Alice")),
        ]);
        assert_eq!(tpl.render(&data).unwrap(), "Hi, Alice!");
    }

    #[test]
    fn dot_notation_nested_access() {
        let tpl = Template::parse("Email: {user.email}").unwrap();
        let mut user = HashMap::new();
        user.insert("email".to_string(), Value::from("a@b.com"));
        let data = data_from(vec![("user", Value::Map(user))]);
        assert_eq!(tpl.render(&data).unwrap(), "Email: a@b.com");
    }

    #[test]
    fn deep_dot_notation() {
        let tpl = Template::parse("{a.b.c}").unwrap();
        let mut c_map = HashMap::new();
        c_map.insert("c".to_string(), Value::from("deep"));
        let mut b_map = HashMap::new();
        b_map.insert("b".to_string(), Value::Map(c_map));
        let data = data_from(vec![("a", Value::Map(b_map))]);
        assert_eq!(tpl.render(&data).unwrap(), "deep");
    }

    #[test]
    fn conditional_truthy() {
        let tpl = Template::parse("{#if show}Hello{/if}").unwrap();
        let data = data_from(vec![("show", Value::from(true))]);
        assert_eq!(tpl.render(&data).unwrap(), "Hello");
    }

    #[test]
    fn conditional_falsy() {
        let tpl = Template::parse("{#if show}Hello{/if}").unwrap();
        let data = data_from(vec![("show", Value::from(false))]);
        assert_eq!(tpl.render(&data).unwrap(), "");
    }

    #[test]
    fn conditional_truthy_string() {
        let tpl = Template::parse("{#if name}Hi, {name}{/if}").unwrap();
        let data = data_from(vec![("name", Value::from("Bob"))]);
        assert_eq!(tpl.render(&data).unwrap(), "Hi, Bob");
    }

    #[test]
    fn conditional_falsy_empty_string() {
        let tpl = Template::parse("{#if name}Hi, {name}{/if}").unwrap();
        let data = data_from(vec![("name", Value::from(""))]);
        assert_eq!(tpl.render(&data).unwrap(), "");
    }

    #[test]
    fn negated_conditional_true() {
        let tpl = Template::parse("{#if !hidden}Visible{/if}").unwrap();
        let data = data_from(vec![("hidden", Value::from(false))]);
        assert_eq!(tpl.render(&data).unwrap(), "Visible");
    }

    #[test]
    fn negated_conditional_false() {
        let tpl = Template::parse("{#if !hidden}Visible{/if}").unwrap();
        let data = data_from(vec![("hidden", Value::from(true))]);
        assert_eq!(tpl.render(&data).unwrap(), "");
    }

    #[test]
    fn negated_conditional_missing_variable() {
        let tpl = Template::parse("{#if !missing}Shown{/if}").unwrap();
        let data = HashMap::new();
        assert_eq!(tpl.render(&data).unwrap(), "Shown");
    }

    #[test]
    fn loop_over_string_list() {
        let tpl = Template::parse("{#each items}{.}, {/each}").unwrap();
        let data = data_from(vec![(
            "items",
            Value::List(vec![
                Value::from("a"),
                Value::from("b"),
                Value::from("c"),
            ]),
        )]);
        assert_eq!(tpl.render(&data).unwrap(), "a, b, c, ");
    }

    #[test]
    fn loop_over_number_list() {
        let tpl = Template::parse("{#each nums}{.} {/each}").unwrap();
        let data = data_from(vec![(
            "nums",
            Value::List(vec![
                Value::from(1i64),
                Value::from(2i64),
                Value::from(3i64),
            ]),
        )]);
        assert_eq!(tpl.render(&data).unwrap(), "1 2 3 ");
    }

    #[test]
    fn loop_over_list_of_maps() {
        let tpl = Template::parse("{#each users}{name} ({email})\n{/each}").unwrap();
        let mut u1 = HashMap::new();
        u1.insert("name".to_string(), Value::from("Alice"));
        u1.insert("email".to_string(), Value::from("alice@example.com"));
        let mut u2 = HashMap::new();
        u2.insert("name".to_string(), Value::from("Bob"));
        u2.insert("email".to_string(), Value::from("bob@example.com"));
        let data = data_from(vec![(
            "users",
            Value::List(vec![Value::Map(u1), Value::Map(u2)]),
        )]);
        assert_eq!(
            tpl.render(&data).unwrap(),
            "Alice (alice@example.com)\nBob (bob@example.com)\n"
        );
    }

    #[test]
    fn filter_upper() {
        let tpl = Template::parse("{name|upper}").unwrap();
        let data = data_from(vec![("name", Value::from("hello"))]);
        assert_eq!(tpl.render(&data).unwrap(), "HELLO");
    }

    #[test]
    fn filter_lower() {
        let tpl = Template::parse("{name|lower}").unwrap();
        let data = data_from(vec![("name", Value::from("HELLO"))]);
        assert_eq!(tpl.render(&data).unwrap(), "hello");
    }

    #[test]
    fn filter_trim() {
        let tpl = Template::parse("{name|trim}").unwrap();
        let data = data_from(vec![("name", Value::from("  hello  "))]);
        assert_eq!(tpl.render(&data).unwrap(), "hello");
    }

    #[test]
    fn escaped_braces() {
        let tpl = Template::parse("Use {{name}} for variables").unwrap();
        let data = HashMap::new();
        assert_eq!(tpl.render(&data).unwrap(), "Use {name} for variables");
    }

    #[test]
    fn missing_variable_error() {
        let tpl = Template::parse("{missing}").unwrap();
        let data = HashMap::new();
        let err = tpl.render(&data).unwrap_err();
        assert_eq!(err, RenderError::MissingVariable("missing".into()));
    }

    #[test]
    fn nested_conditionals() {
        let tpl = Template::parse("{#if a}{#if b}both{/if}{/if}").unwrap();
        let data = data_from(vec![
            ("a", Value::from(true)),
            ("b", Value::from(true)),
        ]);
        assert_eq!(tpl.render(&data).unwrap(), "both");

        let data2 = data_from(vec![
            ("a", Value::from(true)),
            ("b", Value::from(false)),
        ]);
        assert_eq!(tpl.render(&data2).unwrap(), "");
    }

    #[test]
    fn parse_error_unmatched_if() {
        let err = Template::parse("{#if x}hello").unwrap_err();
        assert!(matches!(err, ParseError::UnmatchedBlock(_)));
    }

    #[test]
    fn parse_error_unmatched_each() {
        let err = Template::parse("{#each items}hello").unwrap_err();
        assert!(matches!(err, ParseError::UnmatchedBlock(_)));
    }

    #[test]
    fn parse_error_unexpected_close_if() {
        let err = Template::parse("hello{/if}").unwrap_err();
        assert!(matches!(err, ParseError::UnmatchedBlock(_)));
    }

    #[test]
    fn parse_error_unexpected_close_each() {
        let err = Template::parse("hello{/each}").unwrap_err();
        assert!(matches!(err, ParseError::UnmatchedBlock(_)));
    }

    #[test]
    fn number_display() {
        let tpl = Template::parse("Count: {n}").unwrap();
        let data = data_from(vec![("n", Value::from(42i64))]);
        assert_eq!(tpl.render(&data).unwrap(), "Count: 42");
    }

    #[test]
    fn float_display() {
        let tpl = Template::parse("Pi: {pi}").unwrap();
        let data = data_from(vec![("pi", Value::from(3.14f64))]);
        assert_eq!(tpl.render(&data).unwrap(), "Pi: 3.14");
    }

    #[test]
    fn plain_text_no_tags() {
        let tpl = Template::parse("Just plain text").unwrap();
        let data = HashMap::new();
        assert_eq!(tpl.render(&data).unwrap(), "Just plain text");
    }

    #[test]
    fn conditional_with_list_truthy() {
        let tpl = Template::parse("{#if items}Has items{/if}").unwrap();
        let data = data_from(vec![(
            "items",
            Value::List(vec![Value::from("a")]),
        )]);
        assert_eq!(tpl.render(&data).unwrap(), "Has items");
    }

    #[test]
    fn conditional_with_empty_list_falsy() {
        let tpl = Template::parse("{#if items}Has items{/if}").unwrap();
        let data = data_from(vec![("items", Value::List(vec![]))]);
        assert_eq!(tpl.render(&data).unwrap(), "");
    }

    #[test]
    fn type_error_each_on_non_list() {
        let tpl = Template::parse("{#each name}{.}{/each}").unwrap();
        let data = data_from(vec![("name", Value::from("not a list"))]);
        let err = tpl.render(&data).unwrap_err();
        assert!(matches!(err, RenderError::TypeError(_)));
    }
}
