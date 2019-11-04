use rep::*;

#[derive(CheckRep)]
struct Line {
	// assert equality
	// assert relation
	// #[rep(aseert = "not_eq_zero")]
	// #[rep(assert_false = "eq_zero")]
	// #[rep(assert_eq = 0.0]
	// #[rep(assert_eq = true]
	// #[rep(assert)]	
	// #[rep(assert_eq = "hello")]
	// #[rep(assert_gt = 0.0]
	// #[rep(assert_lt = 0.0]
	// #[rep(assert_ge = 0.0]
	// #[rep(assert_le = 0.0]
	#[rep(assert_gt = 0)]
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32
}

#[check_rep]
impl Line {
	pub fn foo() {
	}
}

fn main() {
	let mut line = Line {
		x1: -20,
		y1: 0,
		x2: 10,
		y2: 10
	};

	// line.foo();
	Line::foo();
}
