use std::collections::HashMap;

use ast::*;
use ast::visit::*;
use class::{self, ClassId};
use ctxt::*;
use error::msg::Msg;
use interner::Name;
use lexer::position::Position;
use mem;
use sym::Sym::{self, SymType};
use ty::BuiltinType;

pub fn check<'ast>(ctxt: &mut Context<'ast>) {
    let mut gdef = GlobalDef {
        ctxt: ctxt
    };

    gdef.visit_ast(ctxt.ast);
}

struct GlobalDef<'x, 'ast: 'x> {
    ctxt: &'x mut Context<'ast>
}

impl<'x, 'ast> Visitor<'ast> for GlobalDef<'x, 'ast> {
    fn visit_class(&mut self, c: &'ast Class) {
        let id = ClassId(self.ctxt.classes.len());
        let cls = class::Class {
            id: id,
            name: c.name,
            ctor: FctId(0),
            props: Vec::new(),
            ast: Some(c),
            size: mem::ptr_width(),
        };

        self.ctxt.classes.push(Box::new(cls));
        let ty = BuiltinType::Class(id);
        let sym = SymType(ty);

        assert!(self.ctxt.cls_defs.insert(c.id, id).is_none());

        if let Some(sym) = self.ctxt.sym.borrow_mut().insert(c.name, sym) {
            report(self.ctxt, c.name, c.pos, sym);
        }
    }

    fn visit_fct(&mut self, f: &'ast Function) {
        let fct = Fct {
            id: FctId(0),
            name: f.name,
            params_types: Vec::new(),
            return_type: BuiltinType::Unit,
            owner_class: None,
            ctor: false,
            kind: FctKind::Source(FctSrc::new(f)),
        };

        if let Err(sym) = self.ctxt.add_fct_to_sym(fct) {
            report(self.ctxt, f.name, f.pos, sym);
        }
    }
}

fn report(ctxt: &Context, name: Name, pos: Position, sym: Sym) {
    let name = ctxt.interner.str(name).to_string();

    let msg = if sym.is_type() {
        Msg::ShadowType(name)
    } else {
        Msg::ShadowFunction(name)
    };

    ctxt.diag.borrow_mut().report(pos, msg);
}
