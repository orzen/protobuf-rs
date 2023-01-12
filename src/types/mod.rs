pub mod enumerate;
pub mod enum_field;
pub mod field;
pub mod field_option;
pub mod import;
pub mod map;
pub mod message;
pub mod oneof;
pub mod opt;
pub mod package;
pub mod proto;
pub mod rpc;
pub mod rpc_option;
pub mod service;
pub mod syntax;

// TODO remove if remain unused
//use crate::types::{
//    enumerate::Enum,
//    enum_field::EnumField,
//    field::Field,
//    field_option::FieldOption,
//    import::Import,
//    map::Map,
//    message::Message,
//    oneof::Oneof,
//    opt::Opt,
//    package::Package,
//    proto::Proto,
//    rpc::Rpc,
//    rpc_option::RpcOption,
//    service::Service,
//    syntax::Syntax,
//};
//
//#[derive(Debug)]
//pub enum Type {
//    Bool(bool),
//    Constant(String),
//    Enum(Enum),
//    EnumField(EnumField),
//    Field(Field),
//    FieldOption(FieldOption),
//    Import(Import),
//    Map(Map),
//    Message(Message),
//    Oneof(Oneof),
//    Opt(Opt),
//    Package(Package),
//    Proto(Proto),
//    Rpc(Rpc),
//    RpcOption(RpcOption),
//    Service(Service),
//    Syntax(Syntax),
//}
