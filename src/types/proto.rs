use super::enumerate::Enum;
use super::import::Import;
use super::message::Message;
use super::opt::Opt;
use super::package::Package;
use super::service::Service;
use super::syntax::Syntax;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Proto {
    enums: Vec<Enum>,
    imports: Vec<Import>,
    messages: Vec<Message>,
    opts: Vec<Opt>,
    package: Package,
    services: Vec<Service>,
    syntax: Syntax,
}

impl Proto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_enum(&mut self, e: Enum) {
        self.enums.push(e)
    }

    pub fn push_import(&mut self, i: Import) {
        self.imports.push(i)
    }

    pub fn push_message(&mut self, m: Message) {
        self.messages.push(m)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o)
    }

    pub fn push_service(&mut self, s: Service) {
        self.services.push(s)
    }

    pub fn set_syntax(&mut self, s: Syntax) {
        self.syntax = s
    }

    pub fn set_package(&mut self, p: Package) {
        self.package = p
    }
}

impl ToString for Proto {
    fn to_string(&self) -> String {
        let mut acc = String::new();

        acc.push_str(self.syntax.to_string().as_str());
        acc.push_str(self.package.to_string().as_str());

        acc
    }
}
