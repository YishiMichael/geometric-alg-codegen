mod glsl;
mod rust;
mod wgsl;

pub use glsl::GLSLLang;
pub use rust::RustLang;
pub use wgsl::WGSLLang;

use crate::ast::{Ast, Record, Stringifier};

pub trait Syntax {
    fn emit_record<A: Ast>(
        &self,
        writer: &mut Writer,
        stringifier: &dyn Stringifier<A>,
        record: &Record<A>,
    ) -> std::io::Result<()>;
}

pub struct Writer<'w> {
    buffer: &'w mut dyn std::io::Write,
    indents: usize,
}

impl<'w> Writer<'w> {
    pub fn new(buffer: &'w mut dyn std::io::Write) -> Self {
        Self { buffer, indents: 0 }
    }

    pub fn buffer(&mut self) -> &mut dyn std::io::Write {
        &mut self.buffer
    }

    pub fn indent(&mut self) {
        self.indents += 1;
    }

    pub fn dedent(&mut self) {
        self.indents -= 1;
    }

    pub fn newline(&mut self) -> std::io::Result<()> {
        writeln!(self.buffer())?;
        let indents = self.indents;
        write!(self.buffer(), "{:width$}", "", width = 4 * indents)?;
        Ok(())
    }
}
