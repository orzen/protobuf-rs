use super::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
	pub name: String,
	pub typ: String,
	pub pos: i32,
	pub opts: Vec<Opt>,
	pub is_repeated: bool,
}

impl Field {
	pub fn new(name: String, typ: String, pos: i32, opts: Vec<Opt>, is_repeated: bool) -> Self {
		Field {
			name,
			typ,
			pos,
			opts,
			is_repeated,
		}
	}
}
