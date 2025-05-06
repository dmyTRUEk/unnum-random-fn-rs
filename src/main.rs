//! unnum radom: ie 3.1415... -> 22/7

#![feature(
	box_patterns,
	iter_intersperse,
)]

#![deny(unreachable_patterns)]

use std::f64::consts::{E as EULER_E, PI};

use rand::{rng, rngs::ThreadRng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const X: float = PI;
const DIFF: float = 1e-6;

fn main() {
	const CORES_N: u32 = 10;
	(0..CORES_N).into_par_iter().for_each(|_| {
		let mut rng = rng();
		loop {
			let expr = Expr::new_random(2, &mut rng);
			let expr_str = expr.to_string();
			let x = expr.eval();
			let diff = diff(X, x);
			if diff < DIFF {
				println!("diff={diff:e},\t{expr_str}");
			}
		}
	});
}

enum Expr {
	Zero,
	One,
	E,
	Pi,
	Int(i64),
	Float(float),
	UnaryMinus(Box<Expr>),
	Add(Vec<Expr>),
	Prod(Vec<Expr>),
	Div { num_denom: Box<(Expr, Expr)> },
	Pow { b_t: Box<(Expr, Expr)> },
	Sqrt { num_rootbase: Box<(Expr, Expr)> },
	Floor(Box<Expr>),
	Ceil(Box<Expr>),
	Round(Box<Expr>),
	Sin(Box<Expr>),
	Cos(Box<Expr>),
	Tan(Box<Expr>),
}
impl Expr {
	fn eval(self) -> float {
		use Expr::*;
		match self {
			Zero => 0.,
			One => 1.,
			E => EULER_E,
			Pi => PI,
			Int(n) => n as float,
			Float(x) => x,
			UnaryMinus(x) => -x.eval(),
			Add(es) => es.into_iter().map(|e| e.eval()).sum(),
			Prod(es) => es.into_iter().map(|e| e.eval()).product(),
			Div { num_denom: box(num, denom) } => num.eval() / denom.eval(),
			Pow { b_t: box(b, t) } => b.eval().powf(t.eval()),
			Sqrt { num_rootbase: box(num, root_base) } => num.eval().powf(1./root_base.eval()),
			Floor(e) => e.eval().floor(),
			Ceil(e) => e.eval().ceil(),
			Round(e) => e.eval().round(),
			Sin(e) => e.eval().sin(),
			Cos(e) => e.eval().cos(),
			Tan(e) => e.eval().tan(),
			// _ => todo!()
		}
	}

	fn new_random(level: u32, rng: &mut ThreadRng) -> Self {
		use Expr::*;
		match level {
			0 => {
				match rng.random_range(0..6) {
					0 => Zero,
					1 => One,
					2 => E,
					// 3 => Pi,
					4 => Int(rng.random_range(-100 ..= 100)),
					// 5 => Float(rng.random_range(-10.0 ..= 10.0)),
					_ => Self::new_random(level, rng),
					// _ => todo!()
				}
			}
			1 => {
				let expr = Box::new(Self::new_random(level-1, rng));
				match rng.random_range(0..7) {
					0 => UnaryMinus(expr),
					1 => Floor(expr),
					2 => Ceil(expr),
					3 => Round(expr),
					4 => Sin(expr),
					5 => Cos(expr),
					6 => Tan(expr),
					_ => todo!()
				}
			}
			_ => {
				let variant = rng.random_range(0..5);
				match variant {
					0 | 1 => {
						let mut exprs = vec![];
						for _ in 0..rng.random_range(2..=5) {
							exprs.push(Self::new_random(level-1, rng));
						}
						match variant {
							0 => Add(exprs),
							1 => Prod(exprs),
							_ => unreachable!()
						}
					}
					2 => Div { num_denom: Box::new((Self::new_random(level-1, rng), Self::new_random(level-1, rng))) },
					3 => Pow { b_t: Box::new((Self::new_random(level-1, rng), Self::new_random(level-1, rng))) },
					4 => Sqrt { num_rootbase: Box::new((Self::new_random(level-1, rng), Self::new_random(level-1, rng))) },
					_ => todo!()
				}
			}
		}
	}

	fn to_string(&self) -> String {
		use Expr::*;
		match self {
			Zero => format!("0"),
			One => format!("1"),
			E => format!("e"),
			Pi => format!("π"),
			Int(n) => format!("{n}"),
			Float(x) => format!("{x}"),
			UnaryMinus(x) => format!("-({})", x.to_string()),
			Add(es) => es.into_iter().map(|e| e.to_string()).intersperse(format!("+")).collect(),
			Prod(es) => es.into_iter().map(|e| e.to_string()).intersperse(format!("*")).collect(),
			Div { num_denom: box(num, denom) } => format!("({})/({})", num.to_string(), denom.to_string()),
			Pow { b_t: box(b, t) } => format!("({})^({})", b.to_string(), t.to_string()),
			Sqrt { num_rootbase: box(num, root_base) } => format!("({})^(1/({}))", num.to_string(), root_base.to_string()),
			Floor(e) => format!("⌊{}⌋", e.to_string()),
			Ceil(e) => format!("⌈{}⌉", e.to_string()),
			Round(e) => format!("⌊{}⌉", e.to_string()),
			Sin(e) => format!("sin({})", e.to_string()),
			Cos(e) => format!("cos({})", e.to_string()),
			Tan(e) => format!("tan({})", e.to_string()),
			// _ => todo!()
		}
	}
}

fn diff(x: float, y: float) -> float {
	(x-y).abs()
}

#[allow(non_camel_case_types)]
type float = f64;

