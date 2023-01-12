pub mod enumerate;
pub mod field;
pub mod import;
pub mod map;
pub mod message;
pub mod oneof;
pub mod opt;
pub mod package;
pub mod proto;
pub mod rpc;
pub mod service;
pub mod syntax;

use crate::types::{
    enumerate::Enum,
    field::Field,
    import::Import,
    map::Map,
    message::Message,
    oneof::Oneof,
    opt::Opt,
    package::Package,
    proto::Proto,
    rpc::RPC,
    service::Service,
    syntax::Syntax,
};

pub enum Type {
    Bool(bool),
    Constant(String),
    Enum(Enum),
    Field(Field),
    Import(Import),
    Map(Map),
    Message(Message),
    Oneof(Oneof),
    Opt(Opt),
    Package(Package),
    Proto(Proto),
    RPC(RPC),
    Service(Service),
    Syntax(Syntax),
}
