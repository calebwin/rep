# `rep`
[![](http://meritbadge.herokuapp.com/rep)](https://crates.io/crates/rep)
[![](https://docs.rs/rep/badge.svg)](https://docs.rs/rep)

`rep` is a tiny utility that lets you easily enforce [representation/class invariants](https://en.wikipedia.org/wiki/Class_invariant) throughout your Rust data structures.

Representation invariants are logical assertions that must hold true for every mutation of your data structure. For example, in your GIS application, you may have the following rep invariant for a `LatLong`.
```rust
self.lat >= -90.0 && self.lat <= 90 && self.long >= -180.0 && self.long <= 180
```

Enforcing representation invariants is easy with `rep`. Adding invariants to your data structures is just 2 easy steps.
1. Define a correct representation (by implementing `CheckRep` either manually or with a macro)
2. Insert runtime checks (either manually or with a macro)

# some examples

We can start off with a simple data structure.
```rust
use rep::*;

pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}
```
The `CheckRep` trait can be implemented. This serves as a definition of correct representation.
```rust
impl CheckRep for Line {
    fn is_correct(&self) -> bool {
        self.x1 != self.x2 && self.y1 != self.y2
    }
}
```
Now we can use the `#[check_rep]` macro to automatically insert calls to `check_rep` at start and end of all methods that are `pub` and mutate `&mut self`. We can also manually make calls to `check_rep` wherever we so desire.
```rust
#[check_rep] // <-- this inserts calls to check_rep at start and end of move_by
impl Line {
    pub fn new() -> Self {
        let new_line = Self {
            x1: -1,
            y1: -1,
            x1: 1,
            y1: 1
        };
        
        new_line.check_rep();
        new_line
    }
    
    pub fn move_by(&mut self, x: i32, y: i32) {
        self.x1 += x;
        self.x2 += x;
        self.y1 += y;
        self.y2 += y;
    }
}
```

# some more examples
For simple representations, we can even derive an implementation of `CheckRep`.
```rust
#[derive(CheckRep)]
struct Circle {
    x: i32,
    y: i32,
    #[rep(assert_gt = 0)]
    #[rep(assert_le = 2048)]
    r: i32,
}
```
```rust
struct Parser {
    #[rep(assert_default)]
    unclosed_delims: (usize, usize, usize) // this is representing (parens, braces, brackets)
}
```

We can recursively check representation and use custom functions per field.
```rust
fn is_health_valid(h: u32) -> bool {
    h > 0 && h < 100
}

#[derive(CheckRep)]
struct Player {
    #[rep(check)]
    position: Point,
    #[rep(assert_with = "is_health_valid")]
    health: u32
}
```

More advanced rep-checking can be done through custom checking.
```rust
fn is_health_valid(h: u32) -> bool {
    h > 0 && h < 100
}

#[derive(CheckRep)]
struct Player {
    #[rep(use_custom)]  // indicates that custom code should be used
    #[rep(check)]
    position: Point,
    #[rep(assert_with = "is_health_valid")]
    health: u32
}

impl CustomCheckRep for Line {
    fn c_correctness(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];
        if self.x2 != self.y2 {
            errors.push(String::from("self.x2 must equal self.y2"));
        }

        if errors.len() == 0 { Ok(()) } else { Err(errors) }
    }
}
```
```rust
struct Player {
    position: Point,
    health: u32
}

impl CheckRep for Player {
    fn correctness(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];
        // your code here...
        if errors.len() == 0 { Ok(()) } else { Err(errors) }
    }
}
```

Once `CheckRep` is implemented, you may use it with the `#[check_rep`, `#[require_rep`, and `#[check_rep` macros.
```rust
// this adds `check_rep` at start and end of all public mutating methods
#[check_rep]
impl Device {
    pub fn turn_on(&mut self) {}
    // require_rep, ensure_rep, check_rep add to start, end, start and end respectively
    #[require_rep]
    pub fn get_voltage(&mut self, p: Position) {}
    #[ensure_rep]
    pub fn actuate(&mut self, p: Position, v: Voltage) {}
    #[check_rep]
    fn do_something(&self) {}
}
```

If a logger is present invariant violation will be logged instead of panicked.

# usage

Just add the following to your `Cargo.toml` file.
```toml
[dependencies]
rep = "0.3.0"
```

Then, in your module.
```rust
use rep::*;
```
