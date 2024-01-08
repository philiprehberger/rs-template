# rs-template

Lightweight string template engine with variables, conditionals, and loops. Zero dependencies.

## Installation

```toml
[dependencies]
philiprehberger-template = "0.1"
```

## Usage

```rust
use philiprehberger_template::{Template, Value};
use std::collections::HashMap;

let tpl = Template::parse("Hello, {name}!").unwrap();
let mut data = HashMap::new();
data.insert("name".into(), Value::from("world"));
assert_eq!(tpl.render(&data).unwrap(), "Hello, world!");
```

### Dot notation

```rust
use philiprehberger_template::{Template, Value};
use std::collections::HashMap;

let tpl = Template::parse("Email: {user.email}").unwrap();
let mut user = HashMap::new();
user.insert("email".into(), Value::from("a@b.com"));
let mut data = HashMap::new();
data.insert("user".into(), Value::Map(user));
assert_eq!(tpl.render(&data).unwrap(), "Email: a@b.com");
```

### Conditionals

```rust
use philiprehberger_template::{Template, Value};
use std::collections::HashMap;

let tpl = Template::parse("{#if logged_in}Welcome back!{/if}").unwrap();
let mut data = HashMap::new();
data.insert("logged_in".into(), Value::from(true));
assert_eq!(tpl.render(&data).unwrap(), "Welcome back!");
```

### Loops

```rust
use philiprehberger_template::{Template, Value};
use std::collections::HashMap;

let tpl = Template::parse("{#each items}{.}, {/each}").unwrap();
let mut data = HashMap::new();
data.insert("items".into(), Value::List(vec![
    Value::from("a"), Value::from("b"), Value::from("c"),
]));
assert_eq!(tpl.render(&data).unwrap(), "a, b, c, ");
```

### Filters

```rust
use philiprehberger_template::{Template, Value};
use std::collections::HashMap;

let tpl = Template::parse("{name|upper}").unwrap();
let mut data = HashMap::new();
data.insert("name".into(), Value::from("hello"));
assert_eq!(tpl.render(&data).unwrap(), "HELLO");
```

## API

| Item | Description |
|------|-------------|
| `Template::parse(s)` | Parse a template string, returns `Result<Template, ParseError>` |
| `template.render(&data)` | Render with data, returns `Result<String, RenderError>` |
| `Value` | Enum: `String`, `Number`, `Bool`, `List`, `Map` |
| `{name}` | Variable placeholder |
| `{user.email}` | Dot-notation nested access |
| `{#if cond}...{/if}` | Conditional block |
| `{#if !cond}...{/if}` | Negated conditional |
| `{#each list}...{/each}` | Loop block (`{.}` for current item) |
| `{name\|filter}` | Filters: `upper`, `lower`, `trim` |
| `{{` | Escaped literal `{` |

## License

MIT
