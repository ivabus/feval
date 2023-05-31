pub mod tasks {
	use std::f64::consts;

	pub fn all(expr: String) -> String {
		convert_constants(exit(expr))
	}
	fn convert_constants(expr: String) -> String {
		expr.replace("math::pi", &consts::PI.to_string()).replace("math::e", &consts::E.to_string())
	}
	fn exit(expr: String) -> String {
		if expr.contains("exit") {
			std::process::exit(0);
		}
		expr
	}
}
