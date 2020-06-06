use rep::*;
#[macro_use]
extern crate log;

fn is_gt_zero(num: i32) -> bool {
    num > 0
}

#[derive(CheckRep)]
struct Point {
    #[rep(assert_eq = 0)]
    x: i32,
    y: i32,
}

#[derive(CheckRep)]
struct Line {
	#[rep(use_custom)]
    // #[rep(assert_default)]
    // #[rep(assert_true)]
    // #[rep(assert_false)]
    // #[rep(assert_eq = 0.0)]
    // #[rep(assert_eq = true)]
    // #[rep(assert_eq = "hello")]
    // #[rep(assert_gt = 0.0)]
    // #[rep(assert_lt = 10.0)]
    // #[rep(assert_ge = 0.0)]
    // #[rep(assert_le = 10.0)]
    #[rep(check)]
    start: Point,
    #[rep(assert_with = "is_gt_zero")]
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl CustomCheckRep for Line {
	fn c_correctness(&self) -> Result<(), Vec<String>> {
		let mut errors = vec![];
		if self.x2 != self.y2 {
			errors.push(String::from("self.x2 must equal self.y2"));
		}

		if errors.len() == 0 {
			Ok(())
		} else {
			Err(errors)
		}
	}
}

impl Line {
    #[ensure_rep]
    pub fn foo(&mut self) {}
}

fn main() {
    // env_logger::init();

    let mut line = Line {
        start: Point { x: 50, y: 50 },
        x1: -20,
        y1: 0,
        x2: 5,
        y2: 10,
    };

    line.foo();
    // Line::foo();
}
