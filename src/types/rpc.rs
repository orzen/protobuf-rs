use super::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct RPC {
	pub name: String,
	pub arg: String,
	pub ret: String,
	pub opts: Vec<Opt>,
	pub stream_arg: bool,
	pub stream_ret: bool,
}

impl RPC {
	pub fn new(
		name: String,
		arg: String,
		ret: String,
		opts: Vec<Opt>,
		stream_arg: bool,
		stream_ret: bool,
	) -> Self {
            RPC { name, arg, ret, opts, stream_arg, stream_ret }
	}
}
