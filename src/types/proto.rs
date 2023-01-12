use super::enumerate::Enum;
use super::import::Import;
use super::message::Message;
use super::opt::Opt;
use super::package::Package;
use super::service::Service;
use super::syntax::Syntax;

#[derive(Clone, Debug, PartialEq)]
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
        let enums: Vec<Enum> = Vec::new();
        let imports: Vec<Import> = Vec::new();
        let messages: Vec<Message> = Vec::new();
        let opts: Vec<Opt> = Vec::new();
        let services: Vec<Service> = Vec::new();
        let syntax = Syntax::new(String::from(""));
        let package = Package::new(String::from(""));

        Proto {
            enums,
            imports,
            messages,
            opts,
            services,
            syntax,
            package,
        }
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
