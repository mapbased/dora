use crate::error::msg::{SemError, SemErrorAndPos};

use dora_parser::lexer::position::Position;

pub struct Diagnostic {
    errors: Vec<SemErrorAndPos>,
}

impl Diagnostic {
    pub fn new() -> Diagnostic {
        Diagnostic { errors: Vec::new() }
    }

    pub fn errors(&self) -> &[SemErrorAndPos] {
        &self.errors
    }

    pub fn report_without_path(&mut self, pos: Position, msg: SemError) {
        self.errors.push(SemErrorAndPos::without_path(pos, msg));
    }

    pub fn report(&mut self, file: String, pos: Position, msg: SemError) {
        self.errors.push(SemErrorAndPos::new(file, pos, msg));
    }

    pub fn report_unimplemented(&mut self, file: String, pos: Position) {
        self.errors
            .push(SemErrorAndPos::new(file, pos, SemError::Unimplemented));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn dump(&self) {
        for err in &self.errors {
            println!("{}", &err.message());
        }
    }
}