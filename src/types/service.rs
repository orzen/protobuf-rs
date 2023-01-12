use super::rpc::RPC;
use super::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    pub name: String,
    pub opts: Vec<Opt>,
    pub rpcs: Vec<RPC>,
}

impl Service {
    pub fn new(name: String) -> Self {
        let opts: Vec<Opt> = Vec::new();
        let rpcs: Vec<RPC> = Vec::new();

        Service { name, opts, rpcs }
    }
    
    pub fn push_opt(&mut self, opt: Opt) {
        self.opts.push(opt)
    }

    pub fn push_rpc(&mut self, rpc: RPC) {
        self.rpcs.push(rpc)
    }
}
