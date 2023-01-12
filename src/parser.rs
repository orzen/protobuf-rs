use std::io::{
    BufReader,
    Cursor,
    Read,
};
use std::thread;
use std::thread::JoinHandle;
use log::warn;
use crate::lexer::Lexer;
use crate::segment::Segment;
use crate::token::{
    Keyword,
    Sym,
};
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

pub struct Parser<T, V> {
    lexer: Lexer<T>,
    handles: Vec<JoinHandle<V>>,
}

// Public functions
impl<T, V> Parser<T, V> {
    pub fn new(buf: BufReader<T>) -> Parser<T, V> {
        let lexer = Lexer::new(buf);

        Parser {
            lexer,
            handles: Vec::new(),
        }
    }
}

impl<T: Read, V> Parser<T, V> {
    // Block parser or proto parser
    fn as_block<F>(&mut self, f: F) -> JoinHandle<V>
    where
        F: FnOnce() -> V,
        F: std::marker::Send + 'static,
        V: std::marker::Send + 'static,
    {
        let block = self.lexer.as_block().unwrap_or_else(|| {
            warn!("empty block parser");
            Segment::new(String::new())
        }).into_inner();
        //let cursor = Cursor::new(block.as_bytes());
        let buf = BufReader::new(Cursor::new(block.as_bytes()) as dyn Read);

        let block_parser: Parser<T, V> = Parser::new(buf);

        thread::spawn(f)
    }
}

// Grammar
impl<T: Read, V> Parser<T, V> {
    fn as_enum(&mut self) -> Option<Enum> {
        // Fetch the name
        let name = self.lexer.feed()?.as_ident()?;
        let enumerate = Enum::new(name);

        // Check for opening curly bracket
        if !self.lexer.feed()?.as_sym()?.eq(&Sym::Curly0) {
            return None;
        }

        // TODO spawn thread and create sublexer
        //let sub = self.lexer.as_sub()?;

        Some(enumerate)
    }

    fn as_import(&mut self) -> Option<Import> {
        if !self.lexer.feed()?.as_sym()?.eq(&Sym::Equal) {
            return None;
        }

        let seg = self.lexer.feed()?;

        let scope = match seg.as_keyword() {
            Some(Keyword::Public) => Some(Keyword::Public),
            Some(Keyword::Weak) => Some(Keyword::Weak),
            _ => None,
        };

        let value = self.lexer.feed()?.as_constant()?;

        Some(Import::new(value, scope))
    }

    fn as_message(&mut self) -> Option<Message> {
        // Fetch the name
        let name = self.lexer.feed()?.as_ident()?;
        let msg = Message::new(name);

        // Check for opening curly bracket
        if !self.lexer.feed()?.as_sym()?.eq(&Sym::Curly0) {
            return None;
        }

        // TODO spawn thread and create sublexer
        //let sub = self.lexer.as_sub()?;

        Some(msg)
    }

    fn as_option(&mut self) -> Option<Opt> {
        let name: String;
        let value: String;

        let seg = self.lexer.feed()?;

        // Check if option is a custom option. Custom option names are wrapped in parentheses.
        let is_custom = seg.as_sym().unwrap_or(Sym::Invalid).eq(&Sym::Par0);

        if is_custom {
            name = self.lexer.feed()?.as_full_ident()?;
            value = self.lexer.feed()?.as_constant()?;
        } else {
            name = seg.as_full_ident()?;
            value = self.lexer.feed()?.as_constant()?;
        }

        // Check for closing parantheses
        if is_custom && !self.lexer.feed()?.as_sym()?.eq(&Sym::Par1) {
            return None;
        }

        Some(Opt::new(name, value))
    }

    fn as_package(&mut self) -> Option<Package> {
        let value = self.lexer.feed()?.as_full_ident()?;

        Some(Package::new(value))
    }

    fn as_syntax(&mut self) -> Option<Syntax> {
        let value = self.lexer.feed()?.as_constant()?;

        if value.eq("proto2") || value.eq("proto3") {
            return Some(Syntax::new(value))
        }

        None
    }

    fn as_service(&mut self) -> Option<Service> {
        let name = self.lexer.feed()?.as_ident()?;
        let mut service = Service::new(name);

        if !self.lexer.feed()?.as_sym()?.eq(&Sym::Par0) {
            return None;
        }

        loop {
            let seg = match self.lexer.feed() {
                Some(s) => s,
                None => break,
            };

            match seg.as_keyword() {
                Some(Keyword::Opt) => service.push_opt(self.as_option()?),
                Some(Keyword::RPC) => service.push_rpc(self.as_rpc()?),
                Some(other) => {
                    warn!("expected service element, got {:?}: {}", other, seg.into_inner());
                    break
                },
                None => {
                    warn!("expected service element, got none: {}", seg.into_inner());
                    break
                }
            }
        }

        Some(service)
    }

    // The following functions are used by the block parser

    fn as_field(&mut self) -> Option<Field> {
        // TODO handle with and without type for enum fields and regular fields
        // TODO check to repeated in regular fields
        None
    }

    fn as_map(&mut self) -> Option<Map> {
        None
    }

    fn as_oneof(&mut self) -> Option<Oneof> {
        None
    }

    fn as_rpc(&mut self) -> Option<RPC> {
        // handle stream
        None
    }

    pub fn as_proto(&mut self) -> Option<Proto> {
        let mut proto = Proto::new();
        let spin = true;

        while spin {
            let seg = match self.lexer.feed() {
                Some(s) => s,
                None => return Some(proto),
            };

            let keyword = match seg.as_keyword() {
                Some(kw) => kw,
                None => return Some(proto),
            };

            match keyword {
                Keyword::Enum => proto.push_enum(self.as_enum()?),
                Keyword::Import => proto.push_import(self.as_import()?),
                Keyword::Message => proto.push_message(self.as_message()?),
                Keyword::Opt => proto.push_opt(self.as_option()?),
                Keyword::Package => proto.set_package(self.as_package()?),
                Keyword::Syntax => proto.set_syntax(self.as_syntax()?),
                // Service
                other => {
                    unreachable!("unhandled keyword {:?}", other);
                }
            }
        }

        Some(Proto::new())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::Cursor;

    use super::*;

    const PROTOBUF: &str = r#"
    foo

    "#;

    #[test]
    fn new_from_file() {
        let cursor = Cursor::new(String::from("foobar"));
        let buf = BufReader::new(cursor);

        Parser::new(buf);
    }
}
