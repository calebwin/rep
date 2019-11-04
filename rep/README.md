# `rep`
Rep is a small tool for checking representation/class invariants. As programmers, we should care about correctness at compile time and at run time. We should care about our data structures being correct. Rep is a small tool that lets you do 2 things.
1. Define a correct representation (a rep/class invariant)
2. Insert runtime checks (rep checks)

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
    fn check_rep(&self) -> bool {
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
        
        assert!(new_line.check_rep());
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

# usage

Just add the following to your `Cargo.toml` file.
```toml
[dependencies]
rep = "0.1.0"
```

Then you can begin defining representations and improving the safety of your software.