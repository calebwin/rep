use rep::*;

#[derive(CheckRep)]
struct Line {
	// #[rep(assert_default)]
	// #[rep(assert_true)]
	// #[rep(assert_false)]
	// #[rep(assert_eq = 0.0]
	// #[rep(assert_eq = true]
	// #[rep(assert_eq = "hello")]
	// #[rep(assert_gt = 0.0]
	// #[rep(assert_lt = 10.0]
	// #[rep(assert_ge = 0.0]
	// #[rep(assert_le = 10.0]
	#[rep(assert_gt = 0)]
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32
}

#[check_rep]
impl Line {
	pub fn foo(&mut self) {
	}
}

fn main() {
	let mut line = Line {
		x1: -20,
		y1: 0,
		x2: 10,
		y2: 10
	};

	line.foo();
	// Line::foo();
}
